mod ast;
mod eval;
mod runtime;
mod parser;

use std::io::{self, Write};

fn main() {
    let mut runtime = runtime::Runtime::new();

    println!("🐚 Welcome to Oyster v0.1 — type `exit` to quit");

    loop {
        print!("🐚 ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();

        if line == "exit" { break; }

        match parser::parse(line) {
            Ok(expr) => {
                match eval::eval(expr, &mut runtime) {
                    Ok(val) => println!("→ {val}"),
                    Err(e) => eprintln!("error: {e}"),
                }
            }
            Err(e) => eprintln!("parse error: {e}"),
        }
    }
}