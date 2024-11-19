use chumsky::{prelude::*, Error};

#[derive(Debug)]
enum Const {
    Const(f64),
}

#[derive(Debug)]
enum Var {
    Var(String),
}

#[derive(Debug)]
enum SExpr {
    Const(Box<Const>),
    Var(Box<Var>),
    Neg(Box<SExpr>),
    Add(Box<SExpr>, Box<SExpr>),
    Sub(Box<SExpr>, Box<SExpr>),
    Mul(Box<SExpr>, Box<SExpr>),
    Div(Box<SExpr>, Box<SExpr>),
}

#[derive(Debug)]
enum BExpr {
    GE(Box<Var>, Box<Const>),
    GT(Box<Var>, Box<Const>),
    LE(Box<Var>, Box<Const>),
    LT(Box<Var>, Box<Const>),
    EQ(Box<Var>, Box<Const>),
}

#[derive(Debug)]
enum Command {
    Skip,
    Seq(Box<Command>, Box<Command>),
    Assign(Box<Var>, Box<SExpr>),
    Input(Box<Const>),
    If(Box<BExpr>, Box<Command>, Box<Command>),
    While(Box<BExpr>, Box<Command>),
}

fn parser() -> impl Parser<char, Command, Error = Simple<char>> {
    let skip = text::keyword("skip").map(|_| Command::Skip).padded();

    let int = text::int(10)
        .map(|s: String| Const::Const(s.parse().unwrap()))
        .padded();

    let ident = text::ident::<char, Simple<char>>()
        .map(|s: String| Var::Var(s))
        .padded();

    let sexpr = recursive(|s_expr: Recursive<'_, char, SExpr, Simple<char>>| {
        let int_expr = int.map(|c| SExpr::Const(Box::new(c))).padded();

        let ident_expr = ident.map(|i| SExpr::Var(Box::new(i))).padded();

        let atom = int_expr
            .or(ident_expr)
            .or(s_expr.delimited_by(just('('), just(')')))
            .padded();

        let op = |c| just(c).padded();

        let unary = op('-')
            .repeated()
            .then(atom)
            .foldr(|_op, rhs| SExpr::Neg(Box::new(rhs)));

        let product = unary
            .clone()
            .then(
                op('*')
                    .to(SExpr::Mul as fn(_, _) -> _)
                    .or(op('/').to(SExpr::Div as fn(_, _) -> _))
                    .then(unary)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let sum = product
            .clone()
            .then(
                op('+')
                    .to(SExpr::Add as fn(_, _) -> _)
                    .or(op('-').to(SExpr::Sub as fn(_, _) -> _))
                    .then(product)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        sum
    });

    let assign = ident
        .then_ignore(just(":="))
        .then(sexpr.clone())
        .map(|(var, then)| Command::Assign(Box::new(var), Box::new(then)))
        .padded();

    skip.or(assign).then_ignore(end())
}
fn eval(expr: &SExpr) -> Result<f64, String> {
    match expr {
        SExpr::Neg(a) => Ok(-eval(a)?),
        SExpr::Add(a, b) => Ok(eval(a)? + eval(b)?),
        SExpr::Sub(a, b) => Ok(eval(a)? - eval(b)?),
        SExpr::Mul(a, b) => Ok(eval(a)? * eval(b)?),
        SExpr::Div(a, b) => Ok(eval(a)? / eval(b)?),
        _ => todo!(), // We'll handle other cases later
    }
}

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    println!("{:?}", parser().parse(src));
}
