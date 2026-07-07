 

use lexer::token::Token;
use lexer::tokenizer::Lexer;
use std::env;

mod lexer;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <contract.sol>");
        return; /* this is a guard clause; it returns a unit type,
        which is the default return type of a function that doesn't return anything.
        It is used to indicate that the function has completed its execution and does not return any value.*/
    }

    let filename = &args[1]; /* you'd think the index is wrong, but it's correct.
    The first argument is the program name, so the second argument is the filename.
    cos 'cargo run' actually speaks to cargo, and cargo speaks to rustc, and rustc speaks to the program.*/

    let contract = std::fs::read_to_string(filename).expect("Could not read contract file");

    let mut lexer = Lexer::new(contract);

    loop {
        let token = lexer.next_token();

        println!("{:?}", token);

        if token == Token::EOF {
            break;
        }
    }
}
