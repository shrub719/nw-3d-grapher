use heapless::Vec;
use crate::{
    constants::limits::*,
    trig::*
};

#[derive(Debug)]
pub enum ParserError {
    ParserError,
    InvalidCharacter,
    Overflow
}

#[derive(Debug)]
pub enum EvalError {
    EvalError,
    Underflow,
    Overflow
}

#[derive(Debug)]
enum Token {
    X, Y, Z,
    Const(f32),
    Add, Sub,
    Mul, Div,
    Pow,
    Sin, Cos, Tan
}

pub struct Expr {
    is_implicit: bool,
    rpn: Vec<Token, MAX_TOKENS>
}
impl Expr {
    pub fn new(expr: &str, is_implicit: bool) -> Result<Self, ParserError> {
        let mut tokens = Vec::new();

        for split in expr.split_whitespace() {
            let token = match split {
                "x" => Token::X, "y" => Token::Y, "z" => Token::Z,
                "+" => Token::Add, "-" => Token::Sub,
                "*" => Token::Mul, "/" => Token::Div,
                "^" => Token::Pow,
                "sin" => Token::Sin, "cos" => Token::Cos, "tan" => Token::Tan,
                _ => Token::Const(
                    split.parse()
                        .map_err(|_| ParserError::InvalidCharacter)?
                )
            };

            tokens.push(token).map_err(|_| ParserError::Overflow)?
        }
        
        Ok(Self { 
            is_implicit,
            rpn: tokens
        })
    }

    pub fn eval(&self, x: f32, y: f32, z: f32) -> Result<f32, EvalError> {
        let mut stack: Vec<f32, PARSE_STACK> = Vec::new();

        for token in self.rpn.iter() {
            let mut result: f32;

            match *token {
                Token::X => result = x, Token::Y => result = y, Token::Z => result = z,
                Token::Const(n) => result = n,

                Token::Add => {
                    let b = stack.pop().ok_or(EvalError::Underflow)?;
                    let a = stack.pop().ok_or(EvalError::Underflow)?;
                    result = a + b;
                },
                Token::Sub => {
                    let b = stack.pop().ok_or(EvalError::Underflow)?;
                    let a = stack.pop().ok_or(EvalError::Underflow)?;
                    result = a - b;
                },
                Token::Mul => {
                    let b = stack.pop().ok_or(EvalError::Underflow)?;
                    let a = stack.pop().ok_or(EvalError::Underflow)?;
                    result = a * b;
                },
                Token::Div => {
                    let b = stack.pop().ok_or(EvalError::Underflow)?;
                    let a = stack.pop().ok_or(EvalError::Underflow)?;
                    result = a / b;
                },

                Token::Pow => {
                    let b = stack.pop().ok_or(EvalError::Underflow)?;
                    let i = b as i32;
                    let a = stack.pop().ok_or(EvalError::Underflow)?;
                    result = 1.0;
                    
                    if i > 0 {
                        for _ in 0..i {
                            result *= a;
                        }
                    } else {
                        for _ in 0..-i {
                            result /= a;
                        }
                    }
                }

                Token::Sin => {
                    let x = stack.pop().ok_or(EvalError::Underflow)?;
                    result = sin(x);
                },
                Token::Cos => {
                    let x = stack.pop().ok_or(EvalError::Underflow)?;
                    result = cos(x);
                },
                Token::Tan => {
                    let x = stack.pop().ok_or(EvalError::Underflow)?;
                    result = sin(x) / cos(x);
                }
            };
            
            stack.push(result)
                .map_err(|_| EvalError::Overflow);
        }

        Ok(stack.pop().ok_or(EvalError::Underflow)?)
    }
}

