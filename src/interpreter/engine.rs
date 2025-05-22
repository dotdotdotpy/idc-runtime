use std::collections::{HashMap, VecDeque};
use crate::parser::ast::AstNode;
use crate::utils::logger::Logger;
use crate::runtime::memory::MemoryStore;
use crate::utils::utils::resolve_key_or_expression;

#[derive(Clone)]
struct PeriodicBlock {
    interval: u32,
    node: AstNode,
}

pub struct Runtime {
    memory: MemoryStore,
    logger: Logger,
    event_registry: HashMap<String, AstNode>,
    execution_queue: VecDeque<AstNode>,
    every_blocks: Vec<PeriodicBlock>,
    tick_counter: u32,
    event_stack: Vec<String>,
    promote_on_tick: bool,
}

impl Runtime {
    pub fn new(trace_enabled: bool) -> Self {
        let mut logger = Logger::new();
        logger.set_verbose(trace_enabled);

        Runtime {
            memory: MemoryStore::new(),
            logger,
            event_registry: HashMap::new(),
            execution_queue: VecDeque::new(),
            every_blocks: Vec::new(),
            tick_counter: 0,
            event_stack: Vec::new(),
            promote_on_tick: true,
        }
    }

    pub fn set_promote_on_tick(&mut self, value: bool) {
        self.promote_on_tick = value;
        self.memory.set_promote_on_tick(value, Some(&mut self.logger));
    }

    pub fn set_promote_always(&mut self, value: bool) {
        self.logger.debug(&format!("Set promote_always = {}", value));
        self.set_promote_on_tick(value);
    }

    pub fn promote_memory_if(&mut self, condition: &str) {
        match resolve_key_or_expression(condition, &self.memory) {
            Ok(val) if val != "0" => {
                self.logger.debug(&format!("[PROMOTE] condition '{}' => true", condition));
                self.memory.promote_all();
            }
            Ok(_) => self.logger.debug(&format!("[PROMOTE] condition '{}' => false", condition)),
            Err(e) => self.logger.warn(&format!("[PROMOTE] Failed to evaluate condition '{}': {}", condition, e)),
        }
    }

    pub fn load_script(&mut self, ast_nodes: Vec<AstNode>) {
        for node in ast_nodes {
            match node.node_type.as_str() {
                "on" => {
                    self.logger.debug(&format!("[EVENT] Registered event '{}'", node.value));
                    self.event_registry.insert(node.value.clone(), node);
                }
                "every" => {
                    if let Ok(interval) = node.value.parse::<u32>() {
                        self.every_blocks.push(PeriodicBlock { interval, node });
                    } else {
                        self.logger.warn(&format!("Invalid interval in 'every {}'", node.value));
                    }
                }
                "set" => {
                    if node.value == "promote_always" {
                        if let Some(flag) = node.modifiers.get(0) {
                            let value = flag.to_lowercase() == "true";
                            self.set_promote_always(value);
                        }
                    }
                }
                _ => {
                    self.execution_queue.push_back(node);
                }
            }
        }
    }

    pub fn run(&mut self) {
        self.logger.info("Runtime started.");
        self.trigger_event("start");
        while let Some(node) = self.execution_queue.pop_front() {
            self.execute_node(&node);
        }
    }

    pub fn tick(&mut self) {
        self.tick_counter += 1;
        println!("[TICK {}]", self.tick_counter);
        self.logger.debug(&format!("Tick #{}", self.tick_counter));
        let periodic = self.every_blocks.clone();

        for block in periodic {
            if self.tick_counter % block.interval == 0 {
                self.logger.trace(&format!("Tick matched interval: {}", block.interval));
                let scope_name = format!("tick:{}", self.tick_counter);
                self.memory.enter_scope(&scope_name, Some(&mut self.logger));

                for child in &block.node.children {
                    self.execute_node(child);
                }

                if self.promote_on_tick {
                    self.logger.trace("[PROMOTE] From tick scope to global...");
                    self.memory.promote_scope(&scope_name, "global", Some(&mut self.logger));
                }

                self.memory.exit_scope(Some(&mut self.logger));
            }
        }

        if self.event_registry.contains_key("tick") {
            self.logger.trace("Triggering generic 'on tick' handler");
            self.trigger_event("tick");
        }
    }

    pub fn promote_memory(&mut self) {
        self.memory.promote_all();
    }

    pub fn trigger_event(&mut self, event_name: &str) {
        if self.event_stack.contains(&event_name.to_string()) {
            self.logger.warn(&format!(
                "Skipping recursive event '{}': already in call stack {:?}",
                event_name, self.event_stack
            ));
            return;
        }

        if let Some(node) = self.event_registry.get(event_name).cloned() {
            self.logger.trace(&format!("Triggering event '{}'", event_name));
            self.logger.trace(&format!("Call stack before: {:?}", self.event_stack));

            let scope_name = format!("event:{}", event_name);
            self.memory.enter_scope(&scope_name, Some(&mut self.logger));
            self.event_stack.push(event_name.to_string());

            for child in &node.children {
                self.execute_node(child);
            }

            if event_name == "start" && self.promote_on_tick {
                self.logger.trace("[PROMOTE] From start event to global...");
                self.memory.promote_scope(&scope_name, "global", Some(&mut self.logger));
            }

            self.event_stack.pop();
            self.memory.exit_scope(Some(&mut self.logger));
            self.logger.trace(&format!("Call stack after: {:?}", self.event_stack));
        } else {
            self.logger.warn(&format!("Attempted to trigger unknown event '{}'", event_name));

            if let Some(missing_node) = self.event_registry.get("missing").cloned() {
                self.logger.warn(&format!("Falling back to 'on missing' handler for '{}'", event_name));

                let scope_name = format!("event:missing:{}", event_name);
                self.memory.enter_scope(&scope_name, Some(&mut self.logger));
                self.memory.set("event_name", event_name, Some(&mut self.logger));
                self.event_stack.push("missing".to_string());

                for child in &missing_node.children {
                    self.execute_node(child);
                }

                self.event_stack.pop();
                self.memory.exit_scope(Some(&mut self.logger));
            } else {
                self.logger.warn("No 'on missing' handler found; trigger ignored.");
            }
        }
    }

    fn execute_node(&mut self, node: &AstNode) {
        self.logger.trace(&format!("Executing node: {} '{}'", node.node_type, node.value));

        match node.node_type.as_str() {
            "if" | "unless" => {
                match resolve_key_or_expression(&node.value, &self.memory) {
                    Ok(result) => {
                        let should_run = if node.node_type == "if" { result != "0" } else { result == "0" };
                        self.logger.trace(&format!("[EVAL] '{}' => {}", node.value, result));
                        if should_run {
                            for child in &node.children {
                                self.execute_node(child);
                            }
                        }
                    }
                    Err(err) => self.logger.warn(&format!("[EVAL] Failed: {}", err)),
                }
            }

            "respond" => {
                let full_text = std::iter::once(&node.value).chain(&node.modifiers).cloned().collect::<Vec<String>>().join(" ");
                let rendered = self.memory.resolve_placeholders(&full_text);
                self.logger.trace(&format!("Respond output: '{}' from scope [{}]", rendered, self.memory.format_scope_chain()));
                println!("{}", rendered);
            }

            "respond_event" => {
                match resolve_key_or_expression(&node.value, &self.memory) {
                    Ok(result) => {
                        self.logger.trace(&format!("Respond_event: '{}' => {} from [{}]", node.value, result, self.memory.format_scope_chain()));
                        println!("{}", result);
                    }
                    Err(err) => self.logger.warn(&format!("Evaluation failed: {}", err)),
                }
            }

            "trigger" => {
                let event_name = self.memory.resolve_placeholders(&node.value).trim_matches('"').to_string();
                self.logger.trace(&format!("Triggering event via 'trigger': {}", event_name));
                self.trigger_event(&event_name);
            }

            "load" => {
                let mut resolved_key = &node.value;
                let mut dest_scope = None;
                let mut i = 0;
                while i < node.modifiers.len() {
                    if node.modifiers[i] == "as" {
                        if let Some(k) = node.modifiers.get(i + 1) {
                            resolved_key = k;
                            i += 1;
                        }
                    } else if node.modifiers[i] == "to" {
                        if let Some(s) = node.modifiers.get(i + 1) {
                            if s == "global" {
                                dest_scope = Some("global");
                            }
                            i += 1;
                        }
                    }
                    i += 1;
                }
                match resolve_key_or_expression(&node.value, &self.memory) {
                    Ok(result) => {
                        let label = dest_scope.unwrap_or("local");
                        self.logger.trace(&format!("[LOAD] Target scope: {}. Set {} = {}", label, resolved_key, result));
                        self.memory.set_target(resolved_key, &result, dest_scope, Some(&mut self.logger));
                    }
                    Err(err) => {
                        self.logger.warn(&format!("[LOAD] Failed to evaluate '{}': {}", node.value, err));
                        let fallback_key = node.modifiers.get(0).unwrap_or(&node.value);
                        self.memory.set_target(fallback_key, &node.value, None, Some(&mut self.logger));
                    }
                }
            }

            "remember" => {
                self.memory.set(&node.value, "0", Some(&mut self.logger));
            }

            "wait" => {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }

            "reflect" => match node.value.as_str() {
                "memory" => self.reflect_memory(node.modifiers.get(0).map(|s| s.as_str()).unwrap_or("flat")),
                "stack" => self.reflect_stack(),
                "log" => self.reflect_log(),
                "event" => self.reflect_event(),
                "flags" => self.reflect_flags(),
                _ => {}
            },

            "stop" => {
                return;
            }

            _ => {
                for child in &node.children {
                    self.execute_node(child);
                }
            }
        }
    }

    pub fn reflect_memory(&self, mode: &str) {
        match mode {
            "json" => {
                if let Ok(json) = serde_json::to_string_pretty(&self.memory.flatten_map()) {
                    println!("[REFLECT memory:json] {}", json);
                }
            }
            "local" => {
                if let Some(scope) = self.memory.top_scope() {
                    println!("[REFLECT memory:local:{}] {:?}", scope.name, scope.data);
                }
            }
            "all" => {
                for (name, map) in self.memory.get_all_scopes() {
                    println!("[REFLECT memory:all:{}] {:?}", name, map);
                }
            }
            _ => println!("[REFLECT memory] {:?}", self.memory.flatten_map()),
        }
    }

    pub fn reflect_stack(&self) {
        println!("[REFLECT stack] {:?}", self.event_stack);
    }

    pub fn reflect_log(&self) {
        println!("[REFLECT log] Trace buffer:");
        for (i, entry) in self.logger.get_trace_logs().iter().enumerate() {
            println!("  {:>3}: {}", i + 1, entry);
        }
    }

    pub fn reflect_event(&self) {
        println!("[REFLECT event] Registered events: {:?}", self.event_registry.keys().collect::<Vec<_>>());
    }

    pub fn reflect_flags(&self) {
        println!("[REFLECT flags] tick_counter: {}, promote_on_tick: {}", self.tick_counter, self.promote_on_tick);
        println!("[REFLECT flags] memory scope chain: [{}]", self.memory.format_scope_chain());
    }
}
