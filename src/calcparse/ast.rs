/// This program contains list of valid AST nodes that can be constructed and also evaluates an AST to compute a value
// Standard lib
use std::error;
use statrs::function::gamma::gamma;

//structs

// List of allowed AST nodes that can be constructed by Parser
// Tokens can be arithmetic operators or a Number
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Modulo(Box<Node>, Box<Node>),
    Caret(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Factorial(Box<Node>),
    Number(f64),
}

// Given an AST, calculate the numeric value.
pub fn eval(expr: Node) -> Result<f64, Box<dyn error::Error>> {
    use self::Node::*;
    match expr {
        Number(i) => Ok(i),
        Add(expr1, expr2) => Ok(eval(*expr1)? + eval(*expr2)?),
        Subtract(expr1, expr2) => Ok(eval(*expr1)? - eval(*expr2)?),
        Multiply(expr1, expr2) => Ok(eval(*expr1)? * eval(*expr2)?),
        Divide(expr1, expr2) => Ok(eval(*expr1)? / eval(*expr2)?),
        Modulo(expr1, expr2) => Ok(eval(*expr1)? % eval(*expr2)?),
        Negative(expr1) => Ok(-(eval(*expr1)?)),
        Caret(expr1, expr2) => Ok(eval(*expr1)?.powf(eval(*expr2)?)),
        Factorial(sub_expr) => {
            let sub_result = eval(*sub_expr)?;
            if sub_result >= 0.0 {
                if (sub_result % 1.0) > 0.0 {
                    Ok(gamma(sub_result + 1.0))
                } else {
                    let mut factorial_result = 1.0;
                    for i in 2..=(sub_result as usize) {
                        factorial_result *= i as f64;
                    }
                    Ok(factorial_result)
                }
            } else {
                if (sub_result % 1.0) == 0.0 {
                    Ok(f64::NAN)
                } else {
                    Ok(gamma(sub_result + 1.0))
                }
            }
        }
    }
}

//Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_expr1() {
        use crate::calcparse::parser::Parser;

        let ast = Parser::new("1+2-3").unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 0.0);
    }
    #[test]
    fn test_expr2() {
        use crate::calcparse::parser::Parser;

        let ast = Parser::new("3+2-1*5/4").unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 3.75);
    }
}
