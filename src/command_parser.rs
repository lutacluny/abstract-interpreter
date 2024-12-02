use chumsky::prelude::*;

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
    CExpr(Const),
    VExpr(Var),
    Neg(Box<SExpr>),
    Add(Box<SExpr>, Box<SExpr>),
    Sub(Box<SExpr>, Box<SExpr>),
    Mul(Box<SExpr>, Box<SExpr>),
    Div(Box<SExpr>, Box<SExpr>),
}

#[derive(Debug, PartialEq)]
pub enum BExpr {
    GE(Var, Const),
    GT(Var, Const),
    LE(Var, Const),
    LT(Var, Const),
    EQ(Var, Const),
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Skip,
    Seq(Box<Command>, Box<Command>),
    Assign(Var, SExpr),
    Input(Var),
    If(BExpr, Box<Command>, Box<Command>),
    While(BExpr, Box<Command>),
}

fn parser() -> impl Parser<char, Command, Error = Simple<char>> {
    let command: Recursive<'_, char, Command, Simple<char>> = recursive(|command| {
        let skip = text::keyword::<_, _, Simple<char>>("skip")
            .padded()
            .map(|_| Command::Skip);

        let cconst = text::int(10)
            .padded()
            .map(|s: String| Const::Const(s.parse().unwrap()));

        let var = text::ident::<char, Simple<char>>()
            .padded()
            .map(|s: String| Var::Var(s));

        let const_expr = cconst.map(SExpr::CExpr);

        let var_expr = var.map(SExpr::VExpr);

        let op = |s: String| just(s).padded();

        let s_expr = recursive(|s_expr| {
            let atom = const_expr
                .or(var_expr)
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

        let b_expr = var
            .then(
                op(">=".to_string())
                    .or(op(">".to_string()))
                    .or(op("=<".to_string()))
                    .or(op("<".to_string()))
                    .or(op("==".to_string())),
            )
            .then(cconst)
            .map(|((v, o), c)| construct_bexpr(&o, v, c));

        let assign = var
            .then_ignore(just(":="))
            .then(s_expr.clone())
            .map(|(var, then)| Command::Assign(var, then));

        let input = text::keyword("input")
            .ignore_then(var.delimited_by(just('('), just(')')))
            .map(Command::Input);

        let cif = text::keyword("if")
            .padded()
            .ignore_then(b_expr.clone().delimited_by(just('('), just(')')))
            .padded()
            .then(command.clone().delimited_by(just('{'), just('}')))
            .padded()
            .then_ignore(text::keyword("else"))
            .padded()
            .then(command.clone().delimited_by(just('{'), just('}')))
            .padded()
            .map(|((b_expr, c1), c2)| Command::If(b_expr, Box::new(c1), Box::new(c2)));

        let cwhile = text::keyword("while")
            .padded()
            .ignore_then(b_expr.clone().delimited_by(just('('), just(')')))
            .padded()
            .then(command.clone().delimited_by(just('{'), just('}')))
            .padded()
            .map(|(b_expr, c)| Command::While(b_expr, Box::new(c)));

        let single_command = input.or(cif).or(skip).or(assign).or(cwhile);

        single_command.separated_by(just(";")).map(|c| {
            c.into_iter()
                .reduce(|acc, c| Command::Seq(Box::new(acc), Box::new(c)))
                .unwrap()
        })
    });

    command.then_ignore(end())
}

pub fn parse(src: &str) -> Command {
    match parser().parse(src) {
        Ok(ast) => ast,
        Err(eval_err) => panic!("{:?}", eval_err),
    }
}

fn construct_bexpr(s: &str, v: Var, c: Const) -> BExpr {
    match s {
        "==" => BExpr::EQ(v, c),
        ">=" => BExpr::GE(v, c),
        ">" => BExpr::GT(v, c),
        "<=" => BExpr::LE(v, c),
        "<" => BExpr::LT(v, c),
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
            Command::Assign(Var::Var("x".to_string()), SExpr::CExpr(Const::Const(50.0)))
        );
    }

    #[test]
    fn input() {
        let program = "input(x)";
        let command = parse(&program);
        assert_eq!(command, Command::Input(Var::Var("x".to_string())));
    }

    #[test]
    fn cif() {
        let program = "if (x == 50) {skip} else {skip}";
        let command = parse(&program);
        assert_eq!(
            command,
            Command::If(
                BExpr::EQ(Var::Var("x".to_string()), Const::Const(50.0)),
                Box::new(Command::Skip),
                Box::new(Command::Skip)
            )
        );
    }

    #[test]
    fn cwhile() {
        let program = "while (x < 10) {skip}";
        let command = parse(&program);
        assert_eq!(
            command,
            Command::While(
                BExpr::LT(Var::Var("x".to_string()), Const::Const(10.0)),
                Box::new(Command::Skip)
            )
        );
    }

    #[test]
    fn seq() {
        let program = "skip;skip";
        let command = parse(&program);
        assert_eq!(
            Command::Seq(Box::new(Command::Skip), Box::new(Command::Skip)),
            command
        );
    }

    #[test]
    fn nested_seq() {
        let program = "while (x < 10) {skip;skip}; skip";
        let command = parse(&program);
        assert_eq!(
            Command::Seq(
                Box::new(Command::While(
                    BExpr::LT(Var::Var("x".to_string()), Const::Const(10.0)),
                    Box::new(Command::Seq(
                        Box::new(Command::Skip),
                        Box::new(Command::Skip)
                    ))
                )),
                Box::new(Command::Skip)
            ),
            command
        );
    }

    #[test]
    #[should_panic]
    fn not_in_language() {
        let program = "while (x < 10) {}";
        parse(&program);
    }
}
