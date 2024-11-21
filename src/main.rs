use crate::command_parser::parse;
pub mod command_parser;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    println!("{:?}", parse(&src));
}
