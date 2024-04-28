use super::token::Token;
use std::iter::Peekable;
use std::str::Chars;

pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(new_expr: &'a str) -> Self {
        Tokenizer {
            expr: new_expr.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let next_char = self.expr.next();

        match next_char {
            Some('@') => Some(Token::Ans),
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            Some('|') => Some(Token::Bar),
            Some('!') => Some(Token::ExclamationMark),
            Some(',') => Some(Token::Comma),
            Some('%') => Some(Token::Modulo),
            Some('π') => Some(Token::Pi),
            Some('⌊') => Some(Token::LeftFloor),
            Some('⌋') => Some(Token::RightFloor),
            Some('⌈') => Some(Token::LeftCeiling),
            Some('⌉') => Some(Token::RightCeiling),
            Some('²') => Some(Token::Pow2),
            Some('³') => Some(Token::Pow3),
            Some('°') => Some(Token::DegToRad),
            Some('0'..='9') => {
                let mut number = next_char?.to_string();
                while let Some(next_char) = self.expr.peek() {
                    if next_char.is_ascii_digit() || next_char == &'.' {
                        number.push(self.expr.next()?);
                    } else if next_char == &'(' {
                        return None;
                    } else {
                        break;
                    }
                }

                Some(Token::Num(number.parse::<f64>().unwrap()))
            }
            Some('a') => {
                if self.expr.clone().take(5).collect::<String>() == "tan2(" {
                    self.expr.by_ref().take(4).for_each(drop);
                    Some(Token::Atan2)
                } else if self.expr.clone().take(4).collect::<String>() == "sin(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::Asin)
                } else if self.expr.clone().take(4).collect::<String>() == "cos(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::Acos)
                } else if self.expr.clone().take(4).collect::<String>() == "tan(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::Atan)
                } else if self.expr.clone().take(4).collect::<String>() == "rsinh(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::Arsinh)
                } else if self.expr.clone().take(4).collect::<String>() == "rcosh(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::Arcosh)
                } else if self.expr.clone().take(4).collect::<String>() == "rtanh(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::Artanh)
                } else if self.expr.clone().take(4).collect::<String>() == "sinh(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::Arsinh)
                } else if self.expr.clone().take(4).collect::<String>() == "cosh(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::Arcosh)
                } else if self.expr.clone().take(4).collect::<String>() == "tanh(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::Artanh)
                } else if self.expr.clone().take(3).collect::<String>() == "bs(" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::Abs)
                } else {
                    None
                }
            }
            Some('c') => {
                if self.expr.clone().take(3).collect::<String>() == "os(" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::Cos)
                } else if self.expr.clone().take(4).collect::<String>() == "osh(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::Cosh)
                } else if self.expr.clone().take(4).collect::<String>() == "eil(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::Ceil)
                } else {
                    None
                }
            }
            Some('f') => {
                if self.expr.clone().take(5).collect::<String>() == "loor(" {
                    self.expr.by_ref().take(4).for_each(drop);
                    Some(Token::Floor)
                } else {
                    None
                }
            }
            Some('e') => {
                if self.expr.clone().take(3).collect::<String>() == "xp(" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::Exp)
                } else if self.expr.clone().take(4).collect::<String>() == "xp2(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::Exp2)
                } else {
                    Some(Token::E)
                }
            }
            Some('l') => {
                if self.expr.clone().take(2).collect::<String>() == "n(" {
                    self.expr.by_ref().take(1).for_each(drop);
                    Some(Token::Ln)
                } else if self.expr.clone().take(3).collect::<String>() == "og(" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::Log)
                } else {
                    None
                }
            }
            Some('m') => {
                if self.expr.clone().take(3).collect::<String>() == "in(" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::Min)
                } else if self.expr.clone().take(3).collect::<String>() == "ax(" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::Max)
                } else {
                    None
                }
            }
            Some('p') => {
                if self.expr.clone().take(1).collect::<String>() == "i" {
                    self.expr.by_ref().take(1).for_each(drop);
                    Some(Token::Pi)
                } else if self.expr.clone().take(3).collect::<String>() == "ow(" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::Pow)
                } else {
                    None
                }
            }
            Some('r') => {
                if self.expr.clone().take(5).collect::<String>() == "ound(" {
                    self.expr.by_ref().take(4).for_each(drop);
                    Some(Token::Round)
                } else if self.expr.clone().take(2).collect::<String>() == "ad" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::RadToDeg)
                } else {
                    None
                }
            }
            Some('s') => {
                if self.expr.clone().take(3).collect::<String>() == "in(" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::Sin)
                } else if self.expr.clone().take(4).collect::<String>() == "inh(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::Sinh)
                } else if self.expr.clone().take(4).collect::<String>() == "qrt(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::Sqrt)
                } else if self.expr.clone().take(4).collect::<String>() == "ign(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::Sign)
                } else if self.expr.clone().take(6).collect::<String>() == "ignum(" {
                    self.expr.by_ref().take(5).for_each(drop);
                    Some(Token::Sign)
                } else if self.expr.clone().take(3).collect::<String>() == "gn(" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::Sign)
                } else {
                    None
                }
            }
            Some('t') => {
                if self.expr.clone().take(3).collect::<String>() == "an(" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::Tan)
                } else if self.expr.clone().take(4).collect::<String>() == "anh(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::Tanh)
                } else if self.expr.clone().take(8).collect::<String>() == "runcate(" {
                    self.expr.by_ref().take(7).for_each(drop);
                    Some(Token::Truncate)
                } else if self.expr.clone().take(5).collect::<String>() == "runc(" {
                    self.expr.by_ref().take(4).for_each(drop);
                    Some(Token::Truncate)
                } else {
                    None
                }
            }
            None => Some(Token::Eof),
            Some(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_number() {
        let mut tokenizer = Tokenizer::new("34");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34.0))
    }
    #[test]
    fn test_decimal_number() {
        let mut tokenizer = Tokenizer::new("34.5");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34.5))
    }
    #[test]
    fn test_sin_function() {
        let mut tokenizer = Tokenizer::new("sin(3.14159)");
        assert_eq!(tokenizer.next().unwrap(), Token::Sin)
    }
    #[test]
    fn test_cos_function() {
        let mut tokenizer = Tokenizer::new("cos(3.14159)");
        assert_eq!(tokenizer.next().unwrap(), Token::Cos)
    }
    #[test]
    fn test_tan_function() {
        let mut tokenizer = Tokenizer::new("tan(3.14159)");
        assert_eq!(tokenizer.next().unwrap(), Token::Tan)
    }
    #[test]
    fn test_atan2_function() {
        let mut tokenizer = Tokenizer::new("atan2(3.14159, 2.1415)");
        assert_eq!(tokenizer.next().unwrap(), Token::Atan2)
    }
}
