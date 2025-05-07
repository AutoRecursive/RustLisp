use std::rc::Rc;
use std::collections::HashMap;
use std::cell::RefCell;
use crate::types::{Value, Environment, car, cdr};
use crate::eval::eval_list;

/// Type definition for Rust functions that can be called from Lisp
pub type RustFunction = fn(Vec<Rc<Value>>) -> Rc<Value>;

/// Global registry of Rust functions that can be called from Lisp
pub struct RustFunctionRegistry {
    functions: RefCell<HashMap<String, RustFunction>>,
}

impl RustFunctionRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        RustFunctionRegistry {
            functions: RefCell::new(HashMap::new()),
        }
    }

    /// Register a Rust function to be callable from Lisp
    pub fn register(&self, name: &str, func: RustFunction) {
        self.functions.borrow_mut().insert(name.to_string(), func);
    }

    /// Get a registered Rust function by name
    pub fn get(&self, name: &str) -> Option<RustFunction> {
        self.functions.borrow().get(name).cloned()
    }
}

// Create a thread-local registry
thread_local! {
    static RUST_FUNCTIONS: RustFunctionRegistry = RustFunctionRegistry::new();
}

/// Register a Rust function to be callable from Lisp
pub fn register_rust_function(name: &str, func: RustFunction) {
    RUST_FUNCTIONS.with(|registry| {
        registry.register(name, func);
    });
}

/// Call a Rust function from Lisp
/// This is the implementation of the 'rust-call' special form
pub fn rust_call(args: Rc<Value>, env: Rc<Environment>) -> Rc<Value> {
    // First argument should be the function name
    if let Value::Symbol(func_name) = &*car(&args) {
        // Evaluate the rest of the arguments
        let eval_args = eval_list(cdr(&args), env);

        // Convert the arguments to a Vec
        let mut arg_vec = Vec::new();
        let mut current = eval_args;
        while let Value::Cons(car_val, cdr_val) = &*current {
            arg_vec.push(car_val.clone());
            current = cdr_val.clone();
        }

        // Look up the function in the registry
        let mut result = Rc::new(Value::Nil);
        RUST_FUNCTIONS.with(|registry| {
            if let Some(func) = registry.get(func_name) {
                // Call the function with the arguments
                result = func(arg_vec);
            } else {
                println!("Rust function '{}' not found", func_name);
            }
        });
        return result;
    }

    println!("Expected function name as first argument");
    Rc::new(Value::Nil)
}

/// Setup the environment with the 'rust-call' special form
pub fn setup_rust_functions(env: Rc<Environment>) {
    env.borrow_mut().insert(
        "rust-call".to_string(),
        Rc::new(Value::Procedure("rust-call".to_string(), rust_call))
    );
}

// Example Rust functions that can be called from Lisp

/// A simple Rust function that adds two numbers
pub fn rust_add(args: Vec<Rc<Value>>) -> Rc<Value> {
    if args.len() < 2 {
        println!("rust-add requires at least two arguments");
        return Rc::new(Value::Nil);
    }

    let mut result = 0.0;
    for arg in args {
        if let Value::Number(n) = &*arg {
            result += n;
        } else {
            println!("rust-add requires numeric arguments");
            return Rc::new(Value::Nil);
        }
    }

    Rc::new(Value::Number(result))
}

/// A Rust function that multiplies two numbers
pub fn rust_multiply(args: Vec<Rc<Value>>) -> Rc<Value> {
    if args.len() < 2 {
        println!("rust-multiply requires at least two arguments");
        return Rc::new(Value::Nil);
    }

    let mut result = 1.0;
    for arg in args {
        if let Value::Number(n) = &*arg {
            result *= n;
        } else {
            println!("rust-multiply requires numeric arguments");
            return Rc::new(Value::Nil);
        }
    }

    Rc::new(Value::Number(result))
}

/// A Rust function that returns the length of a list
pub fn rust_length(args: Vec<Rc<Value>>) -> Rc<Value> {
    if args.len() != 1 {
        println!("rust-length requires exactly one argument");
        return Rc::new(Value::Nil);
    }

    let mut count = 0;
    let mut current = args[0].clone();

    while let Value::Cons(_, cdr_val) = &*current {
        count += 1;
        current = cdr_val.clone();
    }

    Rc::new(Value::Number(count as f64))
}

/// A Rust function that converts a string to uppercase
pub fn rust_uppercase(args: Vec<Rc<Value>>) -> Rc<Value> {
    if args.len() != 1 {
        println!("rust-uppercase requires exactly one argument");
        return Rc::new(Value::Nil);
    }

    if let Value::Symbol(s) = &*args[0] {
        return Rc::new(Value::Symbol(s.to_uppercase()));
    }

    println!("rust-uppercase requires a symbol argument");
    Rc::new(Value::Nil)
}
