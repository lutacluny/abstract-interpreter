use chumsky::{prelude::*, recursive, Error};

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
    CExpr(Box<Const>),
    VExpr(Box<Var>),
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
    let command = recursive(|command| {
        let skip = text::keyword::<_, _, Simple<char>>("skip")
            .map(|_| Command::Skip)
            .padded();

        let int = text::int(10)
            .map(|s: String| Const::Const(s.parse().unwrap()))
            .padded();

        let ident = text::ident::<char, Simple<char>>()
            .map(|s: String| Var::Var(s))
            .padded();

        let int_expr = int.map(|c| SExpr::CExpr(Box::new(c))).padded();

        let ident_expr = ident.map(|i| SExpr::VExpr(Box::new(i))).padded();

        let op = |s: String| just(s).padded();

        let sexpr = recursive(|s_expr| {
            let atom = int_expr
                .or(ident_expr)
                .or(s_expr.delimited_by(just('('), just(')')))
                .padded();

            let unary = op("-".to_string())
                .repeated()
                .then(atom)
                .foldr(|_op, rhs| SExpr::Neg(Box::new(rhs)));

            let product = unary
                .clone()
                .then(
                    op("*".to_string())
                        .to(SExpr::Mul as fn(_, _) -> _)
                        .or(op("/".to_string()).to(SExpr::Div as fn(_, _) -> _))
                        .then(unary)
                        .repeated(),
                )
                .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

            let sum = product
                .clone()
                .then(
                    op("+".to_string())
                        .to(SExpr::Add as fn(_, _) -> _)
                        .or(op("-".to_string()).to(SExpr::Sub as fn(_, _) -> _))
                        .then(product)
                        .repeated(),
                )
                .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

            sum
        });

        let bexpr = ident
            .then(
                op(">=".to_string())
                    .or(op(">".to_string()))
                    .or(op("=<".to_string()))
                    .or(op("<".to_string()))
                    .or(op("==".to_string())),
            )
            .then(int)
            .map(|((v, o), c)| construct_bexpr(&o, v, c));

        let assign = ident
            .then_ignore(just(":="))
            .then(sexpr.clone())
            .map(|(var, then)| Command::Assign(Box::new(var), Box::new(then)))
            .padded();

        let input = text::keyword("input")
            .ignore_then(int.delimited_by(just('('), just(')')).padded())
            .map(|then| Command::Input(Box::new(then)));

        let cif = text::keyword("if")
            .padded()
            .ignore_then(bexpr.clone().delimited_by(just('('), just(')')))
            .padded()
            .then(command.clone().delimited_by(just('{'), just('}')))
            .padded()
            .then_ignore(text::keyword("else"))
            .padded()
            .then(command.clone().delimited_by(just('{'), just('}')))
            .padded()
            .map(|((b_expr, c1), c2)| Command::If(Box::new(b_expr), Box::new(c1), Box::new(c2)));

        skip.or(cif)
    });

    command
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

fn construct_bexpr(s: &str, v: Var, c: Const) -> BExpr {
    match s {
        "==" => BExpr::EQ(Box::new(v), Box::new(c)),
        ">=" => BExpr::GE(Box::new(v), Box::new(c)),
        ">" => BExpr::GT(Box::new(v), Box::new(c)),
        "<=" => BExpr::LE(Box::new(v), Box::new(c)),
        "<" => BExpr::LT(Box::new(v), Box::new(c)),
        _ => todo!(),
    }
}
fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    println!("{:?}", parser().parse(src));
}
