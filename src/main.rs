use std::env;
use std::fs;
use std::process;
mod scanner;
mod token;
mod token_type;
use crate::scanner::Scanner;

fn main() {
    let mut lox = Lox { had_error: false };
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        eprintln!("Usage: loxcraft [script]");
        process::exit(64);
    } else if args.len() == 2 {
        lox.run_file(&args[1]);
    } else {
        lox.run_prompt();
    }
}

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn run_file(&self, path: &str) {
        let source = fs::read_to_string(path).expect("Failed to read file");
        self.run(&source);

        if self.had_error {
            process::exit(65);
        }
    }

    pub fn run_prompt(&mut self) {
        loop {
            let mut line = String::new();
            match std::io::stdin().read_line(&mut line) {
                Ok(_) => self.run(&line),
                Err(_) => break,
            }
            self.had_error = false;
        }
    }

    pub fn run(&self, source: &str) {
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();

        for token in scanner.tokens {
            println!("{}", token.to_string());
        }
    }

    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    pub fn report(&mut self, line: usize, location: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, location, message);
        self.had_error = true;
    }
}
