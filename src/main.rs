use std::env;
use idc_runtime::parser::tokenizer::tokenize_file;
use idc_runtime::parser::ast::build_ast;
use idc_runtime::parser::semantic::analyze_semantics;
use idc_runtime::interpreter::engine::Runtime;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_usage();
        return;
    }

    let trace_enabled = args.contains(&"--trace".to_string());
    let promote_always_flag = parse_promote_always(&args);
    let command = args[1].as_str();
    let file_path = &args[2];
    let tick_count = parse_tick_arg(&args);

    match command {
        "--tokens" => match tokenize_file(file_path) {
            Ok(tokens) => {
                println!("Tokens:");
                for token in tokens {
                    println!("{:#?}", token);
                }
            }
            Err(err) => eprintln!("Tokenization error: {}", err),
        }

        "--ast" => parse_and_execute(file_path, false, false, None, trace_enabled, promote_always_flag),
        "--semantics" => parse_and_execute(file_path, true, false, None, trace_enabled, promote_always_flag),
        "--run" => parse_and_execute(file_path, true, true, tick_count, trace_enabled, promote_always_flag),

        "--reflect" => {
            if args.len() < 4 {
                eprintln!("Usage: --reflect <file> <memory|stack|log> [--as=json] [--trace]");
                return;
            }

            let reflect_type = args[3].as_str();
            let as_json = args.contains(&"--as=json".to_string());
            let mode = if as_json { "json" } else { "flat" };

            match tokenize_file(file_path) {
                Ok(tokens) => match build_ast(&tokens) {
                    Ok(ast_nodes) => {
                        let mut runtime = Runtime::new(trace_enabled);
                        if let Some(val) = promote_always_flag {
                            runtime.set_promote_always(val);
                        }
                        runtime.load_script(ast_nodes);
                        runtime.trigger_event("start");

                        match reflect_type {
                            "memory" => runtime.reflect_memory(mode),
                            "stack" => runtime.reflect_stack(),
                            "log" => runtime.reflect_log(),
                            _ => eprintln!("Unknown reflect type: {}", reflect_type),
                        }
                    }
                    Err(e) => eprintln!("AST build error: {:?}", e),
                },
                Err(e) => eprintln!("File error: {}", e),
            }
        }

        _ => print_usage(),
    }
}

fn parse_and_execute(
    file_path: &str,
    check_semantics: bool,
    auto_run: bool,
    tick_count: Option<u32>,
    trace_enabled: bool,
    promote_always_flag: Option<bool>,
) {
    match tokenize_file(file_path) {
        Ok(tokens) => match build_ast(&tokens) {
            Ok(ast_nodes) => {
                if check_semantics {
                    let warnings = analyze_semantics(&ast_nodes);
                    if !warnings.is_empty() {
                        println!("Semantic Warnings:");
                        for w in warnings {
                            println!("{:#?}", w);
                        }
                        if !auto_run {
                            return;
                        }
                    } else {
                        println!("No semantic issues detected.");
                    }
                }

                if !auto_run {
                    println!("AST:");
                    for node in &ast_nodes {
                        println!("{:#?}", node);
                    }
                    return;
                }

                let mut runtime = Runtime::new(trace_enabled);
                if let Some(val) = promote_always_flag {
                    runtime.set_promote_always(val);
                }
                runtime.load_script(ast_nodes);

                if let Some(n) = tick_count {
                    runtime.trigger_event("start");
                    runtime.promote_memory();
                    for i in 0..n {
                        println!("[TICK {}]", i + 1);
                        runtime.tick();
                    }
                } else {
                    runtime.run();
                }
            }
            Err(e) => eprintln!("AST build error: {:?}", e),
        },
        Err(e) => eprintln!("File error: {}", e),
    }
}

fn parse_tick_arg(args: &[String]) -> Option<u32> {
    if let Some(pos) = args.iter().position(|a| a == "--ticks") {
        if let Some(val) = args.get(pos + 1) {
            return val.parse::<u32>().ok();
        }
    }
    None
}

fn parse_promote_always(args: &[String]) -> Option<bool> {
    for arg in args {
        if let Some(val) = arg.strip_prefix("--promote-always=") {
            return match val.to_lowercase().as_str() {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            };
        }
    }
    None
}

fn print_usage() {
    println!("IDC Runtime CLI");
    println!("Usage:");
    println!("  --tokens <file>                   Show token stream");
    println!("  --ast <file>                      Show AST structure");
    println!("  --semantics <file>                Run semantics check");
    println!("  --run <file>                      Execute file (runs if semantically valid)");
    println!("  --run <file> --ticks <n>          Simulate N ticks using the tick engine");
    println!("  --reflect <file> <command>        Run reflection (memory, stack, log)");
    println!("       Use --as=json with memory    Show memory in JSON mode");
    println!("  --promote-always=true|false       Enable or disable post-tick memory promotion");
    println!("  --trace                           Enable scoped trace logging");
}
