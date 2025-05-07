use std::rc::Rc;
use crate::types::{Value, cons};

pub fn read(input: &str) -> Result<Rc<Value>, String> {
    let mut chars = input.chars().peekable();
    read_expr(&mut chars)
}

fn read_expr<I>(chars: &mut std::iter::Peekable<I>) -> Result<Rc<Value>, String>
where
    I: Iterator<Item = char>,
{
    // Skip whitespace
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }

    match chars.peek() {
        Some(&'(') => {
            chars.next(); // Skip '('
            read_list(chars)
        }
        Some(&'\'') => {
            chars.next(); // Skip '\''
            let expr = read_expr(chars)?;
            Ok(cons(Rc::new(Value::Symbol("quote".to_string())), cons(expr, Rc::new(Value::Nil))))
        }
        Some(_) => read_atom(chars),
        None => Err("Unexpected end of input".to_string()),
    }
}

fn read_list<I>(chars: &mut std::iter::Peekable<I>) -> Result<Rc<Value>, String>
where
    I: Iterator<Item = char>,
{
    // Skip whitespace
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }

    match chars.peek() {
        Some(&')') => {
            chars.next(); // Skip ')'
            Ok(Rc::new(Value::Nil))
        }
        Some(_) => {
            let car = read_expr(chars)?;
            let cdr = read_list(chars)?;
            Ok(cons(car, cdr))
        }
        None => Err("Expected ')' but got end of input".to_string()),
    }
}

fn read_atom<I>(chars: &mut std::iter::Peekable<I>) -> Result<Rc<Value>, String>
where
    I: Iterator<Item = char>,
{
    let mut atom = String::new();
    
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() || c == '(' || c == ')' {
            break;
        }
        atom.push(c);
        chars.next();
    }
    
    if atom.is_empty() {
        return Err("Expected atom but got nothing".to_string());
    }
    
    // Try to parse as number
    if let Ok(n) = atom.parse::<f64>() {
        Ok(Rc::new(Value::Number(n)))
    } else {
        Ok(Rc::new(Value::Symbol(atom)))
    }
}
