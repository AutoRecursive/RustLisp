use std::rc::Rc;
use crate::types::{Value, Environment, cons};
use crate::eval::eval;

/// Calls a Lisp function from Rust code
///
/// # Arguments
///
/// * `func_name` - The name of the Lisp function to call
/// * `args` - A vector of Lisp values to pass as arguments
/// * `env` - The Lisp environment
///
/// # Returns
///
/// The result of the function call as a Lisp value
pub fn call_lisp_function(func_name: &str, args: Vec<Rc<Value>>, env: Rc<Environment>) -> Rc<Value> {
    // Look up the function in the environment
    let func = match env.borrow().get(func_name) {
        Some(f) => f.clone(),
        None => {
            println!("Function '{}' not found in environment", func_name);
            return Rc::new(Value::Nil);
        }
    };

    // Build a Lisp list from the arguments
    let mut arg_list = Rc::new(Value::Nil);
    for arg in args.iter().rev() {
        arg_list = cons(arg.clone(), arg_list);
    }

    // Build the function call expression: (func arg1 arg2 ...)
    let expr = cons(func.clone(), arg_list);

    // Evaluate the expression
    eval(expr, env)
}

/// Converts a Rust f64 to a Lisp number
pub fn rust_to_lisp_number(n: f64) -> Rc<Value> {
    Rc::new(Value::Number(n))
}

/// Converts a Rust string to a Lisp symbol
pub fn rust_to_lisp_symbol(s: &str) -> Rc<Value> {
    Rc::new(Value::Symbol(s.to_string()))
}

/// Converts a Rust boolean to a Lisp boolean
pub fn rust_to_lisp_bool(b: bool) -> Rc<Value> {
    Rc::new(Value::Bool(b))
}

/// Converts a Rust vector to a Lisp list
pub fn rust_to_lisp_list(v: Vec<Rc<Value>>) -> Rc<Value> {
    let mut result = Rc::new(Value::Nil);
    for item in v.iter().rev() {
        result = cons(item.clone(), result);
    }
    result
}

/// Attempts to convert a Lisp value to a Rust f64
pub fn lisp_to_rust_number(v: &Rc<Value>) -> Option<f64> {
    match &**v {
        Value::Number(n) => Some(*n),
        _ => None,
    }
}

/// Attempts to convert a Lisp value to a Rust String
pub fn lisp_to_rust_string(v: &Rc<Value>) -> Option<String> {
    match &**v {
        Value::Symbol(s) => Some(s.clone()),
        _ => None,
    }
}

/// Attempts to convert a Lisp value to a Rust boolean
pub fn lisp_to_rust_bool(v: &Rc<Value>) -> Option<bool> {
    match &**v {
        Value::Bool(b) => Some(*b),
        Value::Nil => Some(false),
        _ => None,
    }
}

/// Attempts to convert a Lisp list to a Rust vector
pub fn lisp_to_rust_vector(v: &Rc<Value>) -> Vec<Rc<Value>> {
    let mut result = Vec::new();
    let mut current = v.clone();

    while let Value::Cons(car_val, cdr_val) = &*current {
        result.push(car_val.clone());
        current = cdr_val.clone();
    }

    result
}

/// Registers a Lisp function in the environment
///
/// This is a helper function to define Lisp functions from Rust
pub fn register_lisp_function(name: &str, params: &str, body: &str, env: Rc<Environment>) -> Result<(), String> {
    // Parse the parameters and body
    let params_expr = crate::parser::read(params)?;
    let body_expr = crate::parser::read(body)?;

    // Create a lambda expression
    let lambda = Rc::new(Value::Lambda(
        params_expr,
        body_expr,
        env.clone(),
    ));

    // Register the function in the environment
    env.borrow_mut().insert(name.to_string(), lambda);

    Ok(())
}
