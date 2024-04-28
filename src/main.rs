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
/*
    println!("Calculator started...");
    println!("Getters: PreviousResult(@)");
    println!("Supported operations: Add(x+y), Subtract(x-y), Multiply(x*y), Divide(x/y), Modulo(x%y), PowerOf(x^y), Abs(|x|), Floor(⌊x⌋), Ceiling(⌈x⌉), Factorial(x!), PowerOf2(x²) and PowerOf3(x³).");
    println!("Supported functions: abs(x), ln(x), sgn(x), sign(x), signum(x), trunc(x), truncate(x), exp(x), exp2(x), log(x, b), pow(x,y), sqrt(x), floor(x), ceil(x), round(x), sin(θ), cos(θ), tan(θ), sinh(θ), cosh(θ), tanh(θ), asin(x), acos(x), atan(x), asinh(x), arsinh(x), acosh(x), arcosh(x), atanh(x), artanh(x) and atan2(y, x).");
    println!("To exit the calculator, write 'exit' or 'close'.");
    println!("Enter your arithmetic expression below:");

*/

fn main() {
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
