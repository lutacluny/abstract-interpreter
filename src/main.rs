pub mod abstractions;
pub mod command_parser;
pub mod interpreter;

use crate::abstractions::sign_abstraction::SignAbstraction;
use crate::command_parser::parse;
use crate::interpreter::MemoryState;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    let c = parse(&src);

    dbg!(&c);

    let mut pre: MemoryState<SignAbstraction> = MemoryState::new();

    let post = pre.interprete_command(&c);

    dbg!(post);
}
