mod neblua;

use std::process::ExitCode;

use neblua::parser;
use neblua::tokenizer;

fn main() -> ExitCode {
    let input = "print(\"hello\", \"world\", nil)";

    let tokens = match tokenizer::tokenize(input) {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("Error: {}", e);
            return ExitCode::FAILURE;
        }
    };

    println!("Tokens:");
    for token in &tokens {
        println!("{}", token);
    }

    println!();

    let ast = parser::parse(&tokens).unwrap();
    println!("AST:");
    println!("{}", ast);

    return ExitCode::SUCCESS;
}
