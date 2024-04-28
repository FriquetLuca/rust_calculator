use std::fmt;

use super::ast::Node;
use super::token::{OperPrec, Token};
use super::tokenizer::Tokenizer;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
    old_answer: f64,
}
impl<'a> Parser<'a> {
    pub fn new(expr: &'a str, old_answer: Option<f64>) -> Result<Self, ParseError> {
        let mut lexer = Tokenizer::new(expr);
        let cur_token = match lexer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        Ok(Parser {
            tokenizer: lexer,
            current_token: cur_token,
            old_answer: old_answer.unwrap_or_default(),
        })
    }
    pub fn parse(&mut self) -> Result<Node, ParseError> {
        let ast = self.generate_ast(OperPrec::DefaultZero);
        match ast {
            Ok(ast) => Ok(ast),
            Err(e) => Err(e),
        }
    }
}
impl<'a> Parser<'a> {
    fn get_next_token(&mut self) -> Result<(), ParseError> {
        let next_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        self.current_token = next_token;
        Ok(())
    }
    fn generate_ast(&mut self, oper_prec: OperPrec) -> Result<Node, ParseError> {
        let mut left_expr = self.parse_number()?;

        while oper_prec < self.current_token.get_oper_prec() {
            if self.current_token == Token::Eof {
                break;
            }
            let right_expr = self.convert_token_to_node(left_expr.clone())?;
            left_expr = right_expr;
        }
        Ok(left_expr)
    }
    fn static_function_n_args(&mut self, n: i32) -> Result<Vec<Node>, ParseError> {
        self.get_next_token()?;
        self.check_paren(Token::LeftParen)?;
        let mut args = Vec::new();
        for i in 0..n {
            let arg_expr = self.generate_ast(OperPrec::DefaultZero)?;
            args.push(arg_expr);
            if i < n - 1 {
                self.check_paren(Token::Comma)?;
            }
        }
        self.check_paren(Token::RightParen)?;
        Ok(args)
    }

    fn _function_n_args(&mut self) -> Result<Vec<Node>, ParseError> {
        self.get_next_token()?;
        self.check_paren(Token::LeftParen)?;
        let mut args = Vec::new();
        loop {
            let arg_expr = self.generate_ast(OperPrec::DefaultZero)?;
            args.push(arg_expr);
            if Token::Comma == self.current_token {
                self.get_next_token()?;
            } else if Token::RightParen == self.current_token {
                self.get_next_token()?;
                break;
            } else {
                return Err(ParseError::InvalidOperator(format!(
                    "Expected either {:?} or {:?}, got {:?}",
                    Token::Comma,
                    Token::RightParen,
                    self.current_token
                )));
            }
        }
        Ok(args)
    }

    fn parse_number(&mut self) -> Result<Node, ParseError> {
        let token = self.current_token.clone();
        match token {
            Token::Ans => {
                self.get_next_token()?;
                Ok(Node::Number(self.old_answer))
            }
            Token::Abs => Ok(Node::Abs(Box::new(
                self.static_function_n_args(1)?[0].clone(),
            ))),
            Token::Floor => Ok(Node::Floor(Box::new(
                self.static_function_n_args(1)?[0].clone(),
            ))),
            Token::Ceil => Ok(Node::Ceil(Box::new(
                self.static_function_n_args(1)?[0].clone(),
            ))),
            Token::Round => Ok(Node::Round(Box::new(
                self.static_function_n_args(1)?[0].clone(),
            ))),
            Token::Sin => Ok(Node::Sin(Box::new(
                self.static_function_n_args(1)?[0].clone(),
            ))),
            Token::Cos => Ok(Node::Cos(Box::new(
                self.static_function_n_args(1)?[0].clone(),
            ))),
            Token::Tan => Ok(Node::Tan(Box::new(
                self.static_function_n_args(1)?[0].clone(),
            ))),
            Token::Sinh => Ok(Node::Sinh(Box::new(
                self.static_function_n_args(1)?[0].clone(),
            ))),
            Token::Cosh => Ok(Node::Cosh(Box::new(
                self.static_function_n_args(1)?[0].clone(),
            ))),
            Token::Tanh => Ok(Node::Tanh(Box::new(
                self.static_function_n_args(1)?[0].clone(),
            ))),
            Token::Asin => Ok(Node::Asin(Box::new(
                self.static_function_n_args(1)?[0].clone(),
            ))),
            Token::Acos => Ok(Node::Acos(Box::new(
                self.static_function_n_args(1)?[0].clone(),
            ))),
            Token::Atan => Ok(Node::Atan(Box::new(
                self.static_function_n_args(1)?[0].clone(),
            ))),
            Token::Arsinh => Ok(Node::Arsinh(Box::new(
                self.static_function_n_args(1)?[0].clone(),
            ))),
            Token::Arcosh => Ok(Node::Arcosh(Box::new(
                self.static_function_n_args(1)?[0].clone(),
            ))),
            Token::Artanh => Ok(Node::Artanh(Box::new(
                self.static_function_n_args(1)?[0].clone(),
            ))),
            Token::Sqrt => Ok(Node::Sqrt(Box::new(
                self.static_function_n_args(1)?[0].clone(),
            ))),
            Token::Exp => Ok(Node::Exp(Box::new(
                self.static_function_n_args(1)?[0].clone(),
            ))),
            Token::Exp2 => Ok(Node::Exp2(Box::new(
                self.static_function_n_args(1)?[0].clone(),
            ))),
            Token::Ln => Ok(Node::Ln(Box::new(
                self.static_function_n_args(1)?[0].clone(),
            ))),
            Token::Sign => Ok(Node::Sign(Box::new(
                self.static_function_n_args(1)?[0].clone(),
            ))),
            Token::Truncate => Ok(Node::Truncate(Box::new(
                self.static_function_n_args(1)?[0].clone(),
            ))),
            Token::Atan2 => {
                let args = self.static_function_n_args(2)?;
                Ok(Node::Atan2(
                    Box::new(args[0].clone()),
                    Box::new(args[1].clone()),
                ))
            }
            Token::Pow => {
                let args = self.static_function_n_args(2)?;
                Ok(Node::Pow(
                    Box::new(args[0].clone()),
                    Box::new(args[1].clone()),
                ))
            }
            Token::Log => {
                let args = self.static_function_n_args(2)?;
                Ok(Node::Log(
                    Box::new(args[0].clone()),
                    Box::new(args[1].clone()),
                ))
            }
            Token::Subtract => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            }
            Token::Add => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::Negative)?;
                Ok(expr)
            }
            Token::Num(i) => {
                self.get_next_token()?;
                Ok(Node::Number(i))
            }
            Token::Pi => {
                self.get_next_token()?;
                Ok(Node::Number(std::f64::consts::PI))
            }
            Token::E => {
                self.get_next_token()?;
                Ok(Node::Number(std::f64::consts::E))
            }
            Token::LeftParen => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::DefaultZero)?;
                self.check_paren(Token::RightParen)?;
                if self.current_token == Token::LeftParen {
                    let right = self.generate_ast(OperPrec::MulDiv)?;
                    return Ok(Node::Multiply(Box::new(expr), Box::new(right)));
                }
                Ok(expr)
            }
            Token::Bar => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::DefaultZero)?;
                self.check_paren(Token::Bar)?;
                Ok(Node::Abs(Box::new(expr)))
            }
            Token::LeftFloor => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::DefaultZero)?;
                self.check_paren(Token::RightFloor)?;
                Ok(Node::Floor(Box::new(expr)))
            }
            Token::LeftCeiling => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::DefaultZero)?;
                self.check_paren(Token::RightCeiling)?;
                Ok(Node::Floor(Box::new(expr)))
            }
            _ => Err(ParseError::UnableToParse(
                "Unknown parsing token for parsing number".to_string(),
            )),
        }
    }
    fn check_paren(&mut self, expected: Token) -> Result<(), ParseError> {
        if expected == self.current_token {
            self.get_next_token()?;
            Ok(())
        } else {
            Err(ParseError::InvalidOperator(format!(
                "Expected {:?}, got {:?}",
                expected, self.current_token
            )))
        }
    }
    fn convert_token_to_node(&mut self, left_expr: Node) -> Result<Node, ParseError> {
        match self.current_token {
            Token::Add => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Subtract => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Subtract(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Multiply => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Divide => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Divide(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Caret => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::Power)?;
                Ok(Node::Caret(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::ExclamationMark => {
                self.get_next_token()?;
                Ok(Node::Factorial(Box::new(left_expr)))
            }
            Token::Pow2 => {
                self.get_next_token()?;
                Ok(Node::Pow2(Box::new(left_expr)))
            }
            Token::Pow3 => {
                self.get_next_token()?;
                Ok(Node::Pow3(Box::new(left_expr)))
            }
            Token::Modulo => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Modulo(Box::new(left_expr), Box::new(right_expr)))
            }
            _ => Err(ParseError::InvalidOperator(format!(
                "Please enter a valid operator {:?}",
                self.current_token
            ))),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnableToParse(String),
    InvalidOperator(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match &self {
            self::ParseError::UnableToParse(e) => e.clone(),
            self::ParseError::InvalidOperator(e) => e.clone(),
        };
        write!(f, "Error in evaluating {}", message)
    }
}

impl std::convert::From<std::boxed::Box<dyn std::error::Error>> for ParseError {
    fn from(_evalerr: std::boxed::Box<dyn std::error::Error>) -> Self {
        ParseError::UnableToParse("Unable to parse".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calcparse::ast::Node::{Add, Number};
    #[test]
    fn test_addition() {
        let mut parser = Parser::new("1+2", None).unwrap();
        let expected = Add(Box::new(Number(1.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
}
