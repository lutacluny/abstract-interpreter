use std::{cmp, collections::HashMap, convert::From, ops};

use crate::command_parser::{BExpr, Command, Const, SExpr, Var};

pub struct Top;
pub struct Bottom;

pub trait HasTop {
    fn top() -> Top;
}

pub trait HasBottom {
    fn bottom() -> Bottom;
}

#[derive(Debug)]
pub struct MemoryState<
    A: ops::Add<Output = A>
        + ops::Neg<Output = A>
        + ops::Sub<Output = A>
        + ops::Mul<Output = A>
        + ops::Div<Output = A>
        + cmp::PartialEq
        + cmp::PartialOrd
        + From<f64>
        + Copy
        + HasTop
        + From<Top>
        + HasBottom
        + From<Bottom>,
> {
    state: HashMap<String, A>,
}

impl<
        A: ops::Add<Output = A>
            + ops::Neg<Output = A>
            + ops::Sub<Output = A>
            + ops::Mul<Output = A>
            + ops::Div<Output = A>
            + cmp::PartialEq
            + cmp::PartialOrd
            + From<f64>
            + Copy
            + HasTop
            + From<Top>
            + HasBottom
            + From<Bottom>,
    > MemoryState<A>
{
    pub fn new() -> MemoryState<A> {
        MemoryState {
            state: HashMap::new(),
        }
    }

    pub fn interprete_command(&mut self, c: &Command) -> &MemoryState<A> {
        match c {
            Command::Skip => (),
            Command::Seq(c1, c2) => {
                self.interprete_command(c1);
                self.interprete_command(c2);
            }
            Command::Assign(Var::Var(ident), sexpr) => {
                let a = self.interprete_sexpr(sexpr);
                self.state.insert(ident.clone(), a);
            }
            Command::Input(Var::Var(ident)) => {
                self.state.insert(ident.clone(), A::top().into());
            }
            Command::If(bexpr, c1, c2) => {
                if self.interprete_bexpr(&bexpr) {
                    self.interprete_command(c1);
                } else {
                    self.interprete_command(c2);
                }
            }
            Command::While(bexpr, c) => {
                while self.interprete_bexpr(&bexpr) {
                    self.interprete_command(c);
                }
            }
        }
        self
    }

    fn interprete_sexpr(&self, sexpr: &SExpr) -> A {
        match sexpr {
            SExpr::CExpr(Const::Const(number)) => number.clone().into(),
            SExpr::VExpr(Var::Var(ident)) => self.get_from_state_or_panic(ident),
            SExpr::Neg(sexpr) => -self.interprete_sexpr(sexpr),
            SExpr::Add(sexpr1, sexpr2) => {
                self.interprete_sexpr(sexpr1) + self.interprete_sexpr(sexpr2)
            }
            SExpr::Sub(sexpr1, sexpr2) => {
                self.interprete_sexpr(sexpr1) - self.interprete_sexpr(sexpr2)
            }
            SExpr::Mul(sexpr1, sexpr2) => {
                self.interprete_sexpr(sexpr1) * self.interprete_sexpr(sexpr2)
            }
            SExpr::Div(sexpr1, sexpr2) => {
                self.interprete_sexpr(sexpr1) / self.interprete_sexpr(sexpr2)
            }
        }
    }

    fn interprete_bexpr(&self, bexpr: &BExpr) -> bool {
        match bexpr {
            BExpr::GE(Var::Var(ident), Const::Const(number)) => {
                self.get_from_state_or_panic(ident) >= number.clone().into()
            }
            BExpr::GT(Var::Var(ident), Const::Const(number)) => {
                self.get_from_state_or_panic(ident) > number.clone().into()
            }
            BExpr::LE(Var::Var(ident), Const::Const(number)) => {
                self.get_from_state_or_panic(ident) <= number.clone().into()
            }
            BExpr::LT(Var::Var(ident), Const::Const(number)) => {
                self.get_from_state_or_panic(ident) < number.clone().into()
            }
            BExpr::EQ(Var::Var(ident), Const::Const(number)) => {
                self.get_from_state_or_panic(ident) == number.clone().into()
            }
        }
    }

    fn get_from_state_or_panic(&self, ident: &String) -> A {
        if let Some(&number) = self.state.get(ident) {
            number
        } else {
            panic!("Var {} is not in the memory state", ident);
        }
    }
}
