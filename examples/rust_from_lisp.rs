use std::rc::Rc;
use rustlisp2::{
    Environment,
    Value,
    eval,
    read,
    print_value,
    setup_environment,
    setup_rust_functions,
    register_rust_function,
};

fn main() {
    let env = setup_environment();
    
    // Setup Rust functions
    setup_rust_functions(env.clone());
    
    // Register some custom Rust functions
    register_custom_functions();
    
    // Define and run some Lisp code that calls Rust functions
    run_lisp_examples(env.clone());
}

fn register_custom_functions() {
    // Register a simple Rust function that squares a number
    register_rust_function("square", |args| {
        if args.len() != 1 {
            println!("square requires exactly one argument");
            return Rc::new(Value::Nil);
        }
        
        if let Value::Number(n) = &*args[0] {
            return Rc::new(Value::Number(n * n));
        }
        
        println!("square requires a numeric argument");
        Rc::new(Value::Nil)
    });
    
    // Register a Rust function that concatenates symbols
    register_rust_function("concat", |args| {
        let mut result = String::new();
        
        for arg in args {
            if let Value::Symbol(s) = &*arg {
                result.push_str(s);
            } else {
                println!("concat requires symbol arguments");
                return Rc::new(Value::Nil);
            }
        }
        
        Rc::new(Value::Symbol(result))
    });
    
    // Register a Rust function that checks if a number is even
    register_rust_function("is-even", |args| {
        if args.len() != 1 {
            println!("is-even requires exactly one argument");
            return Rc::new(Value::Nil);
        }
        
        if let Value::Number(n) = &*args[0] {
            let is_even = (*n as i64) % 2 == 0;
            return Rc::new(Value::Bool(is_even));
        }
        
        println!("is-even requires a numeric argument");
        Rc::new(Value::Nil)
    });
}

fn run_lisp_examples(env: Rc<Environment>) {
    println!("=== Calling Rust Functions from Lisp ===\n");
    
    // Example 1: Call the square function
    let example1 = "(rust-call square 5)";
    println!("Lisp code: {}", example1);
    match read(example1) {
        Ok(expr) => {
            let result = eval(expr, env.clone());
            println!("Result: {}\n", print_value(&result));
        }
        Err(err) => println!("Error: {}\n", err),
    }
    
    // Example 2: Call the concat function
    let example2 = "(rust-call concat hello world)";
    println!("Lisp code: {}", example2);
    match read(example2) {
        Ok(expr) => {
            let result = eval(expr, env.clone());
            println!("Result: {}\n", print_value(&result));
        }
        Err(err) => println!("Error: {}\n", err),
    }
    
    // Example 3: Call the is-even function
    let example3 = "(rust-call is-even 42)";
    println!("Lisp code: {}", example3);
    match read(example3) {
        Ok(expr) => {
            let result = eval(expr, env.clone());
            println!("Result: {}\n", print_value(&result));
        }
        Err(err) => println!("Error: {}\n", err),
    }
    
    // Example 4: Use Rust functions in a more complex Lisp expression
    let example4 = "(define sum-of-squares (lambda (x y) (+ (rust-call square x) (rust-call square y))))";
    println!("Lisp code: {}", example4);
    match read(example4) {
        Ok(expr) => {
            let result = eval(expr, env.clone());
            println!("Result: {}", print_value(&result));
        }
        Err(err) => println!("Error: {}", err),
    }
    
    // Now use the defined function
    let example5 = "(sum-of-squares 3 4)";
    println!("Lisp code: {}", example5);
    match read(example5) {
        Ok(expr) => {
            let result = eval(expr, env.clone());
            println!("Result: {}\n", print_value(&result));
        }
        Err(err) => println!("Error: {}\n", err),
    }
    
    println!("=== End of Examples ===");
}
