use std::rc::Rc;
use rustlisp2::{
    Environment,
    eval,
    read,
    print_value,
    setup_environment,
    call_lisp_function,
    rust_to_lisp_number,
    lisp_to_rust_number,
    register_lisp_function,
};

fn main() {
    let env = setup_environment();
    
    // Demo of interop functionality
    demo_interop(&env);
    
    // Start the REPL
    println!("Example completed. Run the main program to use the REPL.");
}

fn demo_interop(env: &Rc<Environment>) {
    println!("=== Rust-Lisp Interoperability Demo ===");
    
    // First, let's define a function in Lisp
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
    
    // Now, let's call this function from Rust
    println!("\nCalling Lisp 'square' function from Rust with argument 5...");
    let args = vec![rust_to_lisp_number(5.0)];
    let result = call_lisp_function("square", args, env.clone());
    
    // Convert the result back to a Rust value
    match lisp_to_rust_number(&result) {
        Some(n) => println!("Result: {} (Rust f64 value)", n),
        None => println!("Result could not be converted to a number"),
    }
    
    println!("Lisp representation: {}", print_value(&result));
    
    // Let's define a more complex function
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
    println!("\n=== End of Demo ===\n");
}
