use std::collections::VecDeque;
use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq)]
pub enum LogTarget {
    Stdout,
    Stderr,
    Json,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Info,
    Debug,
    Warn,
    Error,
}

pub struct Logger {
    pub enabled: bool,
    pub verbose: bool,
    pub target: LogTarget,
    pub use_color: bool,
    trace_buffer: VecDeque<String>,
    trace_limit: usize,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            enabled: true,
            verbose: true,
            target: LogTarget::Stdout,
            use_color: true,
            trace_buffer: VecDeque::with_capacity(100),
            trace_limit: 100,
        }
    }

    pub fn info(&self, message: &str) {
        if self.enabled {
            self.log(LogLevel::Info, message);
        }
    }

    pub fn debug(&self, message: &str) {
        if self.enabled && self.verbose {
            self.log(LogLevel::Debug, message);
        }
    }

    pub fn trace(&mut self, message: &str) {
        if self.enabled && self.verbose {
            let prefix = "[TRACE]";
            let tagged = if message.contains("[PROMOTE]") || message.contains("[EVAL]") {
                message.to_string()
            } else {
                format!("{prefix} {message}")
            };

            let colored = if self.use_color {
                format!("\x1b[90m{}\x1b[0m", &tagged)
            } else {
                tagged.clone()
            };

            if self.trace_buffer.len() >= self.trace_limit {
                self.trace_buffer.pop_front();
            }
            self.trace_buffer.push_back(tagged.clone());

            match self.target {
                LogTarget::Stdout => println!("{}", colored),
                LogTarget::Stderr => {
                    let _ = writeln!(io::stderr(), "{}", colored);
                }
                LogTarget::Json => {
                    let json = format!(r#"{{"level":"TRACE","message":"{}"}}"#, message);
                    println!("{}", json);
                }
            }
        }
    }

    pub fn warn(&self, message: &str) {
        if self.enabled {
            self.log(LogLevel::Warn, message);
        }
    }

    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }

    pub fn reflect_trace_log(&self) {
        println!("Recent TRACE logs ({}):", self.trace_buffer.len());
        for (i, line) in self.trace_buffer.iter().enumerate() {
            println!("  {:02}: {}", i + 1, line);
        }
    }

    pub fn reflect_trace_filtered(&self, filter: Option<&str>, limit: Option<usize>) {
        let filter = filter.unwrap_or("");
        let max = limit.unwrap_or(usize::MAX);
        let mut count = 0;

        println!("Filtered TRACE logs (limit={}):", max);
        for (i, line) in self.trace_buffer.iter().rev().enumerate() {
            if filter.is_empty() || line.contains(filter) {
                println!("  {:02}: {}", i + 1, line);
                count += 1;
            }
            if count >= max {
                break;
            }
        }
    }

    pub fn get_trace_logs(&self) -> Vec<String> {
        self.trace_buffer.iter().cloned().collect()
    }

    pub fn set_trace_limit(&mut self, limit: usize) {
        self.trace_limit = limit;
        while self.trace_buffer.len() > limit {
            self.trace_buffer.pop_front();
        }
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn set_verbose(&mut self, value: bool) {
        self.verbose = value;
    }

    pub fn set_target(&mut self, target: LogTarget) {
        self.target = target;
    }

    pub fn set_color(&mut self, value: bool) {
        self.use_color = value;
    }

    pub fn log_with_ctx(&self, level: LogLevel, message: &str, scope: &str) {
        let full = format!("[{}] {}", scope, message);
        self.output(level, &full);
    }

    fn log(&self, level: LogLevel, message: &str) {
        self.output(level, message);
    }

    fn output(&self, level: LogLevel, message: &str) {
        let prefix = match level {
            LogLevel::Info => "[INFO]",
            LogLevel::Debug => "[DEBUG]",
            LogLevel::Warn => "[WARN]",
            LogLevel::Error => "[ERROR]",
        };

        let colored = if self.use_color {
            match level {
                LogLevel::Info => format!("\x1b[34m{}\x1b[0m {}", prefix, message),
                LogLevel::Debug => format!("\x1b[36m{}\x1b[0m {}", prefix, message),
                LogLevel::Warn => format!("\x1b[33m{}\x1b[0m {}", prefix, message),
                LogLevel::Error => format!("\x1b[31m{}\x1b[0m {}", prefix, message),
            }
        } else {
            format!("{} {}", prefix, message)
        };

        match self.target {
            LogTarget::Stdout => println!("{}", colored),
            LogTarget::Stderr => {
                let _ = writeln!(io::stderr(), "{}", colored);
            }
            LogTarget::Json => {
                let json = format!(r#"{{"level":"{:?}","message":"{}"}}"#, level, message);
                println!("{}", json);
            }
        }
    }
}
