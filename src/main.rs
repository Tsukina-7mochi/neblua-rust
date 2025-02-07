mod neblua;

use neblua::parser;
use neblua::tokenizer;

fn main() {
    let input = "print(\"hello\", \"world\")";

    let tokens = tokenizer::tokenize(input).unwrap();
    println!("Tokens:");
    for token in &tokens {
        println!("{}", token);
    }

    println!();

    let ast = parser::parse(&tokens).unwrap();
    println!("AST:");
    println!("{}", ast);
}
