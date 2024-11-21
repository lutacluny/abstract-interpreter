use chumsky::{prelude::*, recursive, Error};

#[derive(Debug, PartialEq)]
pub enum Const {
    Const(f64),
}

#[derive(Debug, PartialEq)]
pub enum Var {
    Var(String),
}

#[derive(Debug, PartialEq)]
pub enum SExpr {
    CExpr(Box<Const>),
    VExpr(Box<Var>),
    Neg(Box<SExpr>),
    Add(Box<SExpr>, Box<SExpr>),
    Sub(Box<SExpr>, Box<SExpr>),
    Mul(Box<SExpr>, Box<SExpr>),
    Div(Box<SExpr>, Box<SExpr>),
}

#[derive(Debug, PartialEq)]
pub enum BExpr {
    GE(Box<Var>, Box<Const>),
    GT(Box<Var>, Box<Const>),
    LE(Box<Var>, Box<Const>),
    LT(Box<Var>, Box<Const>),
    EQ(Box<Var>, Box<Const>),
}

#[derive(Debug, PartialEq)]
pub enum Command {
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
            .padded()
            .map(|_| Command::Skip);

        let int = text::int(10)
            .padded()
            .map(|s: String| Const::Const(s.parse().unwrap()));

        let ident = text::ident::<char, Simple<char>>()
            .padded()
            .map(|s: String| Var::Var(s));

        let int_expr = int.map(|c| SExpr::CExpr(Box::new(c)));

        let ident_expr = ident.map(|i| SExpr::VExpr(Box::new(i)));

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
            .map(|(var, then)| Command::Assign(Box::new(var), Box::new(then)));

        let input = text::keyword("input")
            .ignore_then(int.delimited_by(just('('), just(')')))
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

        input.or(cif).or(skip).or(assign)
    });

    command
}

pub fn parse(src: &str) -> Command {
    match parser().parse(src) {
        Ok(ast) => ast,
        Err(eval_err) => panic!("{:?}", eval_err),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip() {
        let program = "skip";
        let command = parse(&program);
        assert_eq!(command, Command::Skip);
    }

    #[test]
    fn assign() {
        let program = "x := 50";
        let command = parse(&program);
        assert_eq!(
            command,
            Command::Assign(
                Box::new(Var::Var("x".to_string())),
                Box::new(SExpr::CExpr(Box::new(Const::Const(50.0))))
            )
        );
    }

    #[test]
    fn input() {
        let program = "input(50)";
        let command = parse(&program);
        assert_eq!(command, Command::Input(Box::new(Const::Const(50.0))));
    }

    #[test]
    fn cif() {
        let program = "if (x == 50) {skip} else {skip}";
        let command = parse(&program);
        assert_eq!(
            command,
            Command::If(
                Box::new(BExpr::EQ(
                    Box::new(Var::Var("x".to_string())),
                    Box::new(Const::Const(50.0))
                )),
                Box::new(Command::Skip),
                Box::new(Command::Skip)
            )
        );
    }
}
