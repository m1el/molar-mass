mod periodic;
mod tokenizer;
use periodic::lookup_element;
use tokenizer::{Token, Tokenizer};
use std::io::BufRead;

fn calc_molar(source: &str) -> Result<f64, String> {
    use Token::*;
    let mut stack = Vec::new();
    let mut mass = 0.0;
    let mut tokenizer = Tokenizer::new(source).peekable();
    while let Some(token) = tokenizer.next() {
        match token {
            Token::Symbol(sym) => {
                let len = if sym[1] == 0 { 1 } else { 2 };
                let sym = std::str::from_utf8(&sym[..len]).unwrap();
                let count = if let Some(&Number(num)) = tokenizer.peek() {
                    tokenizer.next();
                    num
                } else {
                    1
                };
                let el = lookup_element(sym)
                    .ok_or_else(|| format!("invalid symbol: {}", sym))?;
                mass += el.mass * (count as f64);
            }
            Token::Lparen => {
                stack.push(mass);
                mass = 0.0;
            }
            Token::Rparen => {
                let prev = stack.pop()
                    .ok_or_else(|| "unmatched `)` paren.".to_string())?;
                let count = if let Some(&Number(num)) = tokenizer.peek() {
                    tokenizer.next();
                    num
                } else {
                    1
                };
                mass = prev + mass * (count as f64);
            }
            _ => return Err(format!("unexpected token: {:?}", token))
        }
    }
    if stack.is_empty() {
        Ok(mass)
    } else {
        Err("unmatched `(` paren.".to_string())
    }
}

fn main() {
    for line in std::io::stdin().lock().lines()
        .filter_map(Result::ok)
        .filter(|s| !s.is_empty())
    {
        println!("{:?}", calc_molar(&line));
    }
}
