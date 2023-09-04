use std::env;
use std::path::Path;
use std::process;
use regex::Regex;
mod find;

fn main() {
    let file_appender = tracing_appender::rolling::daily("./log", "tracing.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE) 
        .with_writer(non_blocking)
        .with_ansi(false)
        .event_format(format)
        .init(); 

    let args: Vec<String> = env::args().collect();
    tracing::info!("{:?}", &args);

    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        eprintln!("Find files by regex pattern in given paths.\n");
        eprintln!("Usage: myfind [OPTIONS] <path...> <pattern>\n");
        eprintln!("OPTIONS:");
        eprintln!("  -h, --help         Print help");
        eprintln!("  -r, --recursive    Search recursively");
        eprintln!("  -v, --verbose      Print verbose information");
        process::exit(0);
    }

    if args.len() < 3 {
        eprintln!("Usage: myfind [OPTIONS] <path...> <pattern>");
        process::exit(1);
    }

    let recursive = args.contains(&String::from("-r")) || args.contains(&String::from("--recursive"));
    let verbose = args.contains(&String::from("-v")) || args.contains(&String::from("--verbose"));

    let mut paths: Vec<String> = Vec::new();
    let mut reg = Regex::new("").unwrap();
    for arg in args[1..].iter() {
        if !arg.starts_with('-') {
            if Path::new(arg).exists() {
                paths.push(arg.to_string());
            } else {
                reg = match Regex::new(arg) {
                    Ok(re) => re,
                    Err(err) => {
                        eprintln!("\x1b[31mInvalid regex \x1b[0m '{}': {}", arg, err);
                        process::exit(1);
                    }
                }
            }
        }
    }
    
    match find::find(&paths, &reg, verbose, recursive) {
        Ok(matched) => {
            if matched.is_empty() {
                eprintln!("\x1b[31mNo matched files! \x1b[0m");
                process::exit(1);
            } else {
                eprintln!("\x1b[32mMatched files: \x1b[0m");
                for path in matched {
                    eprintln!("{}", path);
                }
            }
        },
        Err(err) => {
            eprintln!("\x1b[31mError: \x1b[0m {}", err);
            process::exit(1);
        }
    }
}