# Abstract Interpreter and Analyzer

This command line program, written in pure Rust, that allows to parse a program into an Abstract Syntax Tree to interprete or analyze it in an abstract manner.

## Build from source

- install [rustup](https://doc.rust-lang.org/cargo/getting-started/installation.html)

- build with release flag `cargo build -r`

## Run 

`./target/release/abstract_interpreter --help`

## Execute tests

`cargo test`

## Comments

### Initialization of Variables

In contrast to the static analysis described in the book, I do not initialize all variables at the beginning with Top. It has the advantage that I do not have to scan for all variables as an initial step. Whenever a hew new variable is encountered, it is mapped to an abstract value that satisfies the statenent where the variable occurs. A site-effect of this procedure is that one single loop unrolling doesn't have any impact to the post-condition, since the loop invariant is calculated directly in the loop body. Because the respective variables have not been initialized before, they are not considered for a join at the end of the loop. This behavior can be observed in the respective [tests](./src/abstractions/interval_abstraction.rs) that implement the example from Figure 5.4 in the book. 

### Intervales

My Intervals do not work on Integers, as in the book, but on floats. 

### Coalescent Product

The analysis uses the coalescent product of the memory state as default. 


### CLI 

No everything that is implemented can be used with the CLI. But more important, everything has proper test cases that are taken directly from the book.