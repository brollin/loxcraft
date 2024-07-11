use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        eprintln!("Usage: loxcraft [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) {
    let source = fs::read_to_string(path).expect("Failed to read file");
    run(&source);
}

fn run_prompt() {
    loop {
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => run(&line),
            Err(_) => break,
        }
    }
}

fn run(source: &str) {
    println!("{}", source);
}
