use std::rc::Rc;
use std::collections::HashMap;
use std::cell::RefCell;
use crate::types::{Value, Environment, car, cdr, cons};

pub fn eval(expr: Rc<Value>, env: Rc<Environment>) -> Rc<Value> {
    match &*expr {
        Value::Symbol(s) => {
            env.borrow().get(s).cloned().unwrap_or_else(|| Rc::new(Value::Nil))
        }
        Value::Cons(_, _) => {
            let func = eval(car(&expr), env.clone());
            match &*func {
                Value::Procedure(_, f) => f(cdr(&expr), env),
                Value::Lambda(params, body, closure_env) => {
                    let args = eval_list(cdr(&expr), env.clone());
                    let new_env = Rc::new(RefCell::new(HashMap::new()));
                    bind_params(params.clone(), args, new_env.clone(), closure_env.clone());
                    eval(body.clone(), new_env)
                }
                _ => Rc::new(Value::Nil),
            }
        }
        _ => expr,
    }
}

pub fn eval_list(exprs: Rc<Value>, env: Rc<Environment>) -> Rc<Value> {
    if let Value::Nil = *exprs {
        return Rc::new(Value::Nil);
    }
    cons(
        eval(car(&exprs), env.clone()),
        eval_list(cdr(&exprs), env),
    )
}

pub fn bind_params(params: Rc<Value>, args: Rc<Value>, env: Rc<Environment>, outer_env: Rc<Environment>) {
    match (&*params, &*args) {
        (Value::Nil, Value::Nil) => {
            // Copy outer environment
            for (k, v) in outer_env.borrow().iter() {
                env.borrow_mut().insert(k.clone(), v.clone());
            }
        }
        (Value::Symbol(name), _) => {
            env.borrow_mut().insert(name.clone(), args);
            // Copy outer environment
            for (k, v) in outer_env.borrow().iter() {
                if k != name {
                    env.borrow_mut().insert(k.clone(), v.clone());
                }
            }
        }
        (Value::Cons(p_car, p_cdr), Value::Cons(a_car, a_cdr)) => {
            bind_params(p_car.clone(), a_car.clone(), env.clone(), outer_env.clone());
            bind_params(p_cdr.clone(), a_cdr.clone(), env, outer_env);
        }
        _ => {}
    }
}
