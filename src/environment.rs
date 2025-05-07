use std::rc::Rc;
use std::collections::HashMap;
use std::cell::RefCell;
use crate::types::{Value, Environment, car, cdr};
use crate::eval::{eval, eval_list};

pub fn setup_environment() -> Rc<Environment> {
    let env = Rc::new(RefCell::new(HashMap::new()));
    
    // Add primitives
    env.borrow_mut().insert("nil".to_string(), Rc::new(Value::Nil));
    env.borrow_mut().insert("#t".to_string(), Rc::new(Value::Bool(true)));
    
    // Add primitive procedures
    env.borrow_mut().insert("quote".to_string(), 
        Rc::new(Value::Procedure("quote".to_string(), |args, _| car(&args))));
    
    env.borrow_mut().insert("+".to_string(), 
        Rc::new(Value::Procedure("+".to_string(), |args, env| {
            let args = eval_list(args, env);
            let mut result = 0.0;
            let mut current = args;
            while let Value::Cons(car, cdr) = &*current {
                if let Value::Number(n) = &**car {
                    result += n;
                }
                current = cdr.clone();
            }
            Rc::new(Value::Number(result))
        })));
    
    env.borrow_mut().insert("define".to_string(), 
        Rc::new(Value::Procedure("define".to_string(), |args, env| {
            if let Value::Symbol(name) = &*car(&args) {
                let value = eval(car(&cdr(&args)), env.clone());
                env.borrow_mut().insert(name.clone(), value.clone());
                return value;
            }
            Rc::new(Value::Nil)
        })));
    
    env.borrow_mut().insert("lambda".to_string(), 
        Rc::new(Value::Procedure("lambda".to_string(), |args, env| {
            Rc::new(Value::Lambda(
                car(&args),
                car(&cdr(&args)),
                env.clone(),
            ))
        })));
    
    // Add more primitive procedures
    env.borrow_mut().insert("-".to_string(), 
        Rc::new(Value::Procedure("-".to_string(), |args, env| {
            let args = eval_list(args, env);
            if let Value::Cons(first, rest) = &*args {
                if let Value::Number(n) = &**first {
                    let mut result = *n;
                    let mut current = rest.clone();
                    while let Value::Cons(car, cdr) = &*current {
                        if let Value::Number(n) = &**car {
                            result -= n;
                        }
                        current = cdr.clone();
                    }
                    return Rc::new(Value::Number(result));
                }
            }
            Rc::new(Value::Number(0.0))
        })));
    
    env.borrow_mut().insert("*".to_string(), 
        Rc::new(Value::Procedure("*".to_string(), |args, env| {
            let args = eval_list(args, env);
            let mut result = 1.0;
            let mut current = args;
            while let Value::Cons(car, cdr) = &*current {
                if let Value::Number(n) = &**car {
                    result *= n;
                }
                current = cdr.clone();
            }
            Rc::new(Value::Number(result))
        })));
    
    env.borrow_mut().insert("/".to_string(), 
        Rc::new(Value::Procedure("/".to_string(), |args, env| {
            let args = eval_list(args, env);
            if let Value::Cons(first, rest) = &*args {
                if let Value::Number(n) = &**first {
                    let mut result = *n;
                    let mut current = rest.clone();
                    while let Value::Cons(car, cdr) = &*current {
                        if let Value::Number(n) = &**car {
                            if *n != 0.0 {
                                result /= n;
                            }
                        }
                        current = cdr.clone();
                    }
                    return Rc::new(Value::Number(result));
                }
            }
            Rc::new(Value::Number(0.0))
        })));
    
    env.borrow_mut().insert("=".to_string(), 
        Rc::new(Value::Procedure("=".to_string(), |args, env| {
            let args = eval_list(args, env);
            if let Value::Cons(first, rest) = &*args {
                if let Value::Number(n1) = &**first {
                    if let Value::Cons(second, _) = &**rest {
                        if let Value::Number(n2) = &**second {
                            return if n1 == n2 {
                                Rc::new(Value::Bool(true))
                            } else {
                                Rc::new(Value::Bool(false))
                            };
                        }
                    }
                }
            }
            Rc::new(Value::Bool(false))
        })));
    
    env.borrow_mut().insert("<".to_string(), 
        Rc::new(Value::Procedure("<".to_string(), |args, env| {
            let args = eval_list(args, env);
            if let Value::Cons(first, rest) = &*args {
                if let Value::Number(n1) = &**first {
                    if let Value::Cons(second, _) = &**rest {
                        if let Value::Number(n2) = &**second {
                            return if n1 < n2 {
                                Rc::new(Value::Bool(true))
                            } else {
                                Rc::new(Value::Bool(false))
                            };
                        }
                    }
                }
            }
            Rc::new(Value::Bool(false))
        })));
    
    env.borrow_mut().insert(">".to_string(), 
        Rc::new(Value::Procedure(">".to_string(), |args, env| {
            let args = eval_list(args, env);
            if let Value::Cons(first, rest) = &*args {
                if let Value::Number(n1) = &**first {
                    if let Value::Cons(second, _) = &**rest {
                        if let Value::Number(n2) = &**second {
                            return if n1 > n2 {
                                Rc::new(Value::Bool(true))
                            } else {
                                Rc::new(Value::Bool(false))
                            };
                        }
                    }
                }
            }
            Rc::new(Value::Bool(false))
        })));
    
    env
}
