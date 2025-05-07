use std::rc::Rc;
use crate::types::Value;

pub fn print_value(value: &Rc<Value>) -> String {
    match &**value {
        Value::Nil => "()".to_string(),
        Value::Bool(true) => "#t".to_string(),
        Value::Bool(false) => "#f".to_string(),
        Value::Number(n) => format!("{}", n),
        Value::Symbol(s) => s.clone(),
        Value::Procedure(name, _) => format!("<procedure:{}>", name),
        Value::Lambda(_, _, _) => "<lambda>".to_string(),
        Value::Cons(_, _) => {
            let mut result = String::from("(");
            let mut is_first = true;
            let mut current = value.clone();
            
            while let Value::Cons(car, cdr) = &*current {
                if !is_first {
                    result.push_str(" ");
                }
                result.push_str(&print_value(car));
                is_first = false;
                
                match &**cdr {
                    Value::Nil => {},
                    Value::Cons(_, _) => {
                        current = cdr.clone();
                        continue;
                    },
                    _ => {
                        result.push_str(" . ");
                        result.push_str(&print_value(cdr));
                    }
                }
                break;
            }
            
            result.push_str(")");
            result
        }
    }
}
