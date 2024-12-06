pub mod abstractions;
pub mod command_parser;
pub mod interpreter;

use crate::abstractions::interval_abstraction::IntervalAbstraction;
use crate::abstractions::sign_abstraction::SignAbstraction;
use crate::command_parser::parse;
use crate::interpreter::MemoryState;

use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(
    name = "Abstract Interpreter",
    version = "1.0",
    author = "Friedrich Hartmann <hartmann.friedrich@gmx.net>",
    about = "Simple abstract interpreter and analyzer"
)]
struct Cli {
    #[arg(
        short,
        long,
        help = "Path to the program file that needs to be processed."
    )]
    program: String,

    #[arg(
        short,
        long,
        help = "The mode of operation. Options:\n- parse: Only parses the program.\n- interprete: Interprets the program.\n- analyze: Analyzes the program."
    )]
    mode: String,

    #[arg(
        short,
        long,
        required_if_eq("mode", "interprete"),
        required_if_eq("mode", "analyze"),
        help = "The abstraction to use. Options:\n- interval: Interval abstraction.\n- sign: Sign abstraction.\n(Only required for 'interprete' or 'analyze' modes.)"
    )]
    abstraction: Option<String>,
}

fn main() {
    let args = Cli::parse();

    let program_path = args.program;
    let mode = args.mode;
    let abstraction = args.abstraction;

    let src = fs::read_to_string(program_path).unwrap_or_else(|err| {
        eprintln!("Error reading file: {}", err);
        std::process::exit(1);
    });

    match mode.as_str() {
        "parse" => {
            println!("Parsing the program...");
            let c = parse(&src);
            println!("Parse result: {:?}", c);
        }
        "interprete" => {
            println!("Interpreting the program...");
            let c = parse(&src);

            match abstraction {
                Some(abstraction) => match abstraction.as_str() {
                    "interval" => {
                        let mut pre: MemoryState<IntervalAbstraction> = MemoryState::new();
                        let post = pre.interprete_command(&c);
                        println!("Interpretation result: {:?}", post);
                    }
                    "sign" => {
                        let mut pre: MemoryState<SignAbstraction> = MemoryState::new();
                        let post = pre.interprete_command(&c);
                        println!("Interpretation result: {:?}", post);
                    }
                    _ => {
                        eprintln!("Invalid abstraction specified. Use 'sign' or 'interval'.");
                        std::process::exit(1);
                    }
                },
                None => (),
            }
        }
        "analyze" => {
            println!("Analyzing the program...");
            let c = parse(&src);

            match abstraction {
                Some(abstraction) => match abstraction.as_str() {
                    "interval" => {
                        let mut pre: MemoryState<IntervalAbstraction> = MemoryState::new();
                        let post = pre.analyze_command(&c);
                        println!("Interpretation result: {:?}", post);
                    }
                    "sign" => {
                        let mut pre: MemoryState<SignAbstraction> = MemoryState::new();
                        let post = pre.analyze_command(&c);
                        println!("Interpretation result: {:?}", post);
                    }
                    _ => {
                        eprintln!("Invalid abstraction specified. Use 'sign' or 'interval'.");
                        std::process::exit(1);
                    }
                },
                None => (),
            }
        }
        _ => {
            eprintln!("Invalid mode specified. Use 'parse', 'interprete', or 'analyze'.");
            std::process::exit(1);
        }
    }
}
