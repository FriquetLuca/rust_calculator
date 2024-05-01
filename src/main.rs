mod calcparse;

use calcparse::ast;
use calcparse::parser::{ParseError, Parser};
use std::io;

fn evaluate(expr: String, old_answer: f64) -> Result<f64, ParseError> {
    let expr = expr.split_whitespace().collect::<String>();
    let mut math_parser = Parser::new(&expr, Some(old_answer))?;
    let ast = math_parser.parse()?;
    println!("The generated AST is {:?}", ast);

    Ok(ast::eval(ast)?)
}

fn main() {
    println!("Calculator started...");
    let mut old_eval = 0.0;
    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let cleaned_input = input.split_whitespace().collect::<String>();
                if (cleaned_input == "exit") || (cleaned_input == "close") {
                    break;
                }
                match evaluate(cleaned_input, old_eval) {
                    Ok(val) => {
                        old_eval = val;
                        println!("= {}\n", val)
                    }
                    Err(err) => {
                        println!("{}\nPlease enter valid expression\n", err);
                    }
                };
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
