mod calcparse;

use calcparse::ast;
use calcparse::parser::{ParseError, Parser};
use std::io;

fn evaluate(expr: String, old_answer: f64, debug: bool) -> Result<f64, ParseError> {
    let expr = expr.split_whitespace().collect::<String>();
    let mut math_parser = Parser::new(&expr, Some(old_answer))?;
    let ast = math_parser.parse()?;
    if debug {
        println!("{:?}", ast);
    }
    Ok(ast::eval(ast)?)
}

fn main() {
    println!("Calculator started...");
    let mut old_eval = 0.0;
    let mut debug = false;
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let cleaned_input = input.split_whitespace().collect::<String>();
                if (cleaned_input == "exit") || (cleaned_input == "close") {
                    break;
                } else if cleaned_input == "debug" {
                    debug = !debug;
                    println!("Debugging is now set to: {:?}", debug);
                    continue;
                }
                match evaluate(cleaned_input, old_eval, debug) {
                    Ok(val) => {
                        old_eval = val;
                        println!("= {:?}", val)
                    }
                    Err(err) => {
                        println!("{:?}\nPlease enter valid expression.", err);
                    }
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
