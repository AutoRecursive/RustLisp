use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    Symbol(String),
    Cons(Rc<Value>, Rc<Value>),
    Procedure(String, fn(Rc<Value>, Rc<Environment>) -> Rc<Value>),
    Lambda(Rc<Value>, Rc<Value>, Rc<Environment>),
}

pub type Environment = RefCell<HashMap<String, Rc<Value>>>;

pub fn cons(car: Rc<Value>, cdr: Rc<Value>) -> Rc<Value> {
    Rc::new(Value::Cons(car, cdr))
}

pub fn car(pair: &Rc<Value>) -> Rc<Value> {
    match &**pair {
        Value::Cons(car, _) => car.clone(),
        _ => Rc::new(Value::Nil),
    }
}

pub fn cdr(pair: &Rc<Value>) -> Rc<Value> {
    match &**pair {
        Value::Cons(_, cdr) => cdr.clone(),
        _ => Rc::new(Value::Nil),
    }
}
