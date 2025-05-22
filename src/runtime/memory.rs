use std::collections::HashMap;
use crate::interpreter::math::resolve_math_placeholders;
use crate::utils::logger::Logger;

#[derive(Clone)]
pub struct MemoryScope {
    pub name: String,
    pub data: HashMap<String, String>,
}

pub struct MemoryStore {
    stack: Vec<MemoryScope>,
    promote_on_tick: bool,
}

impl MemoryStore {
    pub fn new() -> Self {
        MemoryStore {
            stack: vec![MemoryScope {
                name: "global".to_string(),
                data: HashMap::new(),
            }],
            promote_on_tick: false,
        }
    }

    pub fn set_promote_on_tick(&mut self, enable: bool, mut logger: Option<&mut Logger>) {
        self.promote_on_tick = enable;
        if let Some(ref mut log) = logger {
            log.debug(&format!("[FLAG] promote_on_tick = {}", enable));
        }
    }

    pub fn enter_scope(&mut self, name: &str, mut logger: Option<&mut Logger>) {
        self.stack.push(MemoryScope {
            name: name.to_string(),
            data: HashMap::new(),
        });

        if let Some(ref mut log) = logger {
            log.trace(&format!("[SCOPE] Entered: [{}]", self.format_scope_chain()));
        }
    }

    pub fn exit_scope(&mut self, mut logger: Option<&mut Logger>) {
        if self.stack.len() > 1 {
            let exited_scope = self.stack.pop().unwrap();

            if let Some(ref mut log) = logger {
                log.trace(&format!(
                    "[SCOPE] Exited: [{}] → [{}]",
                    exited_scope.name,
                    self.format_scope_chain()
                ));
            }

            if self.promote_on_tick {
                let from = &exited_scope.name;
                let to = "global";
                self.promote_scope(from, to, logger.as_deref_mut());
                if let Some(ref mut log) = logger {
                    log.trace("[PROMOTE] Auto-promoted after scope exit (promote_on_tick)");
                }
            }
        }
    }

    pub fn set(&mut self, key: &str, value: &str, mut logger: Option<&mut Logger>) {
        if let Some(current_scope) = self.stack.last_mut() {
            let previous = current_scope.data.insert(key.to_string(), value.to_string());

            if let Some(ref mut log) = logger {
                let scope = self.format_scope_chain();
                match previous {
                    Some(old) if old != value => {
                        log.trace(&format!("[MEM] Updated: [{}] {} = {} (was {})", scope, key, value, old));
                    }
                    None => {
                        log.trace(&format!("[MEM] Created: [{}] {} = {}", scope, key, value));
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        for scope in self.stack.iter().rev() {
            if let Some(value) = scope.data.get(key) {
                return Some(value.clone());
            }
        }
        None
    }

    pub fn resolve_placeholders(&self, input: &str) -> String {
        let memory = self.flatten_scoped();
        resolve_math_placeholders(input, &memory)
    }

    pub fn flatten_scoped(&self) -> HashMap<String, String> {
        let mut flat = HashMap::new();
        for scope in self.stack.iter() {
            for (k, v) in &scope.data {
                flat.entry(k.clone()).or_insert(v.clone());
            }
        }
        flat
    }

    pub fn flatten_map(&self) -> HashMap<String, String> {
        let mut flat = HashMap::new();
        for scope in self.stack.iter().rev() {
            for (k, v) in &scope.data {
                flat.insert(k.clone(), v.clone());
            }
        }
        flat
    }

    pub fn promote_all(&mut self) {
        if self.stack.len() >= 2 {
            let top_data = self.stack.last().map(|s| s.data.clone()).unwrap_or_default();

            if let Some(global) = self.stack.first_mut() {
                for (key, val) in top_data {
                    global.data.insert(key, val);
                }
            }
        }
    }

    pub fn promote_scope(&mut self, from: &str, to: &str, mut logger: Option<&mut Logger>) {
        let from_scope = self.stack.iter().find(|s| s.name == from).cloned();
        let to_scope = self.stack.iter_mut().find(|s| s.name == to);

        if let (Some(from), Some(to)) = (from_scope, to_scope) {
            for (k, v) in from.data {
                to.data.insert(k, v);
            }

            if let Some(ref mut log) = logger {
                log.trace(&format!("[PROMOTE] {} → {}", from.name, to.name));
            }
        } else if let Some(ref mut log) = logger {
            log.warn(&format!("[PROMOTE] Failed: {} → {} (missing scope)", from, to));
        }
    }

    pub fn promote_if<F>(&mut self, condition: F, mut logger: Option<&mut Logger>)
    where
        F: FnOnce(&HashMap<String, String>) -> bool,
    {
        if let Some(top) = self.stack.last() {
            if condition(&top.data) {
                self.promote_all();
                if let Some(ref mut log) = logger {
                    log.trace("[PROMOTE] Conditional promotion triggered");
                }
            }
        }
    }

    pub fn current_scope_chain(&self) -> Vec<String> {
        self.stack.iter().map(|s| s.name.clone()).collect()
    }

    pub fn format_scope_chain(&self) -> String {
        self.stack
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<&str>>()
            .join(" > ")
    }

    pub fn get_local_map(&self) -> HashMap<String, String> {
        self.stack.last().map(|s| s.data.clone()).unwrap_or_default()
    }

    pub fn get_all_scopes(&self) -> Vec<(String, HashMap<String, String>)> {
        self.stack.iter().map(|s| (s.name.clone(), s.data.clone())).collect()
    }

    pub fn top_scope(&self) -> Option<&MemoryScope> {
        self.stack.last()
    }

    pub fn all_scopes(&self) -> Vec<(&str, &HashMap<String, String>)> {
        self.stack.iter().map(|s| (s.name.as_str(), &s.data)).collect()
    }

    pub fn set_target(
        &mut self,
        key: &str,
        value: &str,
        target_scope: Option<&str>,
        mut logger: Option<&mut Logger>,
    ) {
        let scope_name = target_scope.unwrap_or("local");
        let target = match scope_name {
            "local" => self.stack.last_mut(),
            "global" => self.stack.first_mut(),
            _ => None,
        };

        if let Some(scope) = target {
            scope.data.insert(key.to_string(), value.to_string());
            if let Some(ref mut log) = logger {
                log.trace(&format!("[MEM] Target set: [{}] {} = {}", scope_name, key, value));
            }
        }
    }

    pub fn scope_contains(&self, key: &str) -> bool {
        self.stack.last().map_or(false, |s| s.data.contains_key(key))
    }
}
