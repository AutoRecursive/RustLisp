use std::io::{self, Write, BufRead};
use std::rc::Rc;

// Import from our library
use rustlisp2::{
    Environment,
    eval,
    read,
    print_value,
    setup_environment,
    setup_rust_functions,
};

fn main() {
    // Setup environment
    let env = setup_environment();

    // Setup Rust functions
    setup_rust_functions(env.clone());

    // Print welcome message
    println!("RustLisp 🦀λ - A tiny Lisp interpreter");
    println!("Type 'exit' to quit");
    println!("");

    // Start the REPL
    repl(env);
}

fn repl(env: Rc<Environment>) {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    println!("RustLisp 🦀λ - A tiny Lisp interpreter");

    loop {
        print!("🦀λ> ");
        stdout.flush().unwrap();

        let mut input = String::new();
        if stdin.lock().read_line(&mut input).is_err() || input.trim() == "exit" {
            break;
        }

        match read(&input) {
            Ok(expr) => {
                let result = eval(expr, env.clone());
                println!("{}", print_value(&result));
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}
