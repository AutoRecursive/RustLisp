use std::io::{self, Write, BufRead};
use std::rc::Rc;

// Import from our library
use rustlisp2::{
    Environment,
    Value,
    eval,
    read,
    print_value,
    setup_environment,
    setup_rust_functions,
    register_rust_function,
    register_lisp_function,
    call_lisp_function,
    rust_to_lisp_number,
    lisp_to_rust_number,
    rust_add,
    rust_multiply,
    rust_length,
    rust_uppercase,
};

fn main() {
    let env = setup_environment();
    
    // Setup Rust functions
    setup_rust_functions(env.clone());
    
    // Register some example Rust functions
    register_rust_functions();
    
    // Demo Lisp -> Rust interop
    demo_lisp_to_rust(&env);
    
    // Demo Rust -> Lisp interop
    demo_rust_to_lisp(&env);
    
    // Start the interactive REPL
    println!("\nStarting interactive REPL. Try using the functions demonstrated above.");
    repl(env);
}

fn register_rust_functions() {
    // Register our example Rust functions
    register_rust_function("rust-add", rust_add);
    register_rust_function("rust-multiply", rust_multiply);
    register_rust_function("rust-length", rust_length);
    register_rust_function("rust-uppercase", rust_uppercase);
    
    // You can also register custom functions inline
    register_rust_function("rust-square", |args| {
        if args.len() != 1 {
            println!("rust-square requires exactly one argument");
            return Rc::new(Value::Nil);
        }
        
        if let Value::Number(n) = &*args[0] {
            return Rc::new(Value::Number(n * n));
        }
        
        println!("rust-square requires a numeric argument");
        Rc::new(Value::Nil)
    });
}

fn demo_lisp_to_rust(env: &Rc<Environment>) {
    println!("=== Demo: Calling Rust from Lisp ===");
    println!("You can call Rust functions from Lisp using the 'rust-call' special form.\n");
    
    // Example 1: Simple Rust function call
    let example1 = "(rust-call rust-add 1 2 3)";
    println!("Lisp code: {}", example1);
    match read(example1) {
        Ok(expr) => {
            let result = eval(expr, env.clone());
            println!("Result: {}\n", print_value(&result));
        }
        Err(err) => println!("Error: {}\n", err),
    }
    
    // Example 2: Using Rust functions in Lisp expressions
    let example2 = "(+ (rust-call rust-square 3) (rust-call rust-square 4))";
    println!("Lisp code: {}", example2);
    match read(example2) {
        Ok(expr) => {
            let result = eval(expr, env.clone());
            println!("Result: {}\n", print_value(&result));
        }
        Err(err) => println!("Error: {}\n", err),
    }
    
    // Example 3: Defining a Lisp function that uses Rust functions
    let example3 = "(define pythagoras (lambda (a b) (sqrt (+ (rust-call rust-square a) (rust-call rust-square b)))))";
    println!("Lisp code: {}", example3);
    match read(example3) {
        Ok(expr) => {
            let result = eval(expr, env.clone());
            println!("Result: {}", print_value(&result));
        }
        Err(err) => println!("Error: {}", err),
    }
    
    // Add sqrt function for the example
    match register_lisp_function(
        "sqrt", 
        "(x)", 
        "(* x 0.5)", // This is not a real sqrt, just for demo
        env.clone()
    ) {
        Ok(_) => {},
        Err(e) => println!("Error registering function: {}", e),
    }
    
    // Now use the defined function
    let example4 = "(pythagoras 3 4)";
    println!("Lisp code: {}", example4);
    match read(example4) {
        Ok(expr) => {
            let result = eval(expr, env.clone());
            println!("Result: {}\n", print_value(&result));
        }
        Err(err) => println!("Error: {}\n", err),
    }
    
    println!("=== End of Lisp -> Rust Demo ===\n");
}

fn demo_rust_to_lisp(env: &Rc<Environment>) {
    println!("=== Demo: Calling Lisp from Rust ===");
    println!("You can define and call Lisp functions from Rust code.\n");
    
    // First, define a Lisp function
    println!("Defining a Lisp function 'square' from Rust...");
    match register_lisp_function(
        "square", 
        "(x)", 
        "(* x x)", 
        env.clone()
    ) {
        Ok(_) => println!("Function 'square' registered successfully!"),
        Err(e) => println!("Error registering function: {}", e),
    }
    
    // Now, call this function from Rust
    println!("\nCalling Lisp 'square' function from Rust with argument 5...");
    let args = vec![rust_to_lisp_number(5.0)];
    let result = call_lisp_function("square", args, env.clone());
    
    // Convert the result back to a Rust value
    match lisp_to_rust_number(&result) {
        Some(n) => println!("Result: {} (Rust f64 value)", n),
        None => println!("Result could not be converted to a number"),
    }
    
    println!("Lisp representation: {}", print_value(&result));
    
    // Define a more complex function
    println!("\nDefining a more complex Lisp function 'sum-of-squares'...");
    match register_lisp_function(
        "sum-of-squares",
        "(x y)",
        "(+ (* x x) (* y y))",
        env.clone()
    ) {
        Ok(_) => println!("Function 'sum-of-squares' registered successfully!"),
        Err(e) => println!("Error registering function: {}", e),
    }
    
    // Call the complex function
    println!("\nCalling 'sum-of-squares' with arguments 3 and 4...");
    let args = vec![
        rust_to_lisp_number(3.0),
        rust_to_lisp_number(4.0),
    ];
    let result = call_lisp_function("sum-of-squares", args, env.clone());
    
    match lisp_to_rust_number(&result) {
        Some(n) => println!("Result: {} (Rust f64 value)", n),
        None => println!("Result could not be converted to a number"),
    }
    
    println!("Lisp representation: {}", print_value(&result));
    println!("\n=== End of Rust -> Lisp Demo ===\n");
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
