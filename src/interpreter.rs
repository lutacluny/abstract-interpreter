use core::f64;
use std::fmt::Debug;
use std::{cmp, collections::HashMap, convert::From, ops};

use crate::command_parser::{BExpr, Command, Const, SExpr, Var};

pub struct Top;
pub struct Bottom;

pub trait AbstractProperties<A> {
    fn top() -> Top;
    fn bottom() -> Bottom;
    fn sat(a: &A, bexpr: &BExpr) -> bool;
    fn inclusion(a0: &A, a1: &A) -> bool;
    fn join(a0: &A, a1: &A) -> A;
    fn refine(a: &A, bexpr: &BExpr) -> A;
    fn widen(a0: &A, a1: &A) -> A;
}

#[derive(Debug, Clone, PartialEq)]
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
        + AbstractProperties<A>
        + From<Top>
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
            + AbstractProperties<A>
            + From<Top>
            + From<Bottom>
            + Debug,
    > MemoryState<A>
{
    pub fn new() -> MemoryState<A> {
        MemoryState {
            state: HashMap::new(),
        }
    }

    pub fn from_state(state: HashMap<String, A>) -> MemoryState<A> {
        MemoryState { state }
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

    fn interprete_sexpr(&mut self, sexpr: &SExpr) -> A {
        match sexpr {
            SExpr::CExpr(Const::Const(number)) => number.clone().into(),
            SExpr::VExpr(Var::Var(ident)) => self.get_from_state_or_insert_default(ident),
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

    fn interprete_bexpr(&mut self, bexpr: &BExpr) -> bool {
        match bexpr {
            BExpr::GE(Var::Var(ident), Const::Const(number)) => {
                self.get_from_state_or_insert_default(ident) >= number.clone().into()
            }
            BExpr::GT(Var::Var(ident), Const::Const(number)) => {
                self.get_from_state_or_insert_default(ident) > number.clone().into()
            }
            BExpr::LE(Var::Var(ident), Const::Const(number)) => {
                self.get_from_state_or_insert_default(ident) <= number.clone().into()
            }
            BExpr::LT(Var::Var(ident), Const::Const(number)) => {
                self.get_from_state_or_insert_default(ident) < number.clone().into()
            }
            BExpr::EQ(Var::Var(ident), Const::Const(number)) => {
                self.get_from_state_or_insert_default(ident) == number.clone().into()
            }
            BExpr::NE(Var::Var(ident), Const::Const(number)) => {
                self.get_from_state_or_insert_default(ident) != number.clone().into()
            }
        }
    }

    fn get_from_state_or_insert_default(&mut self, ident: &String) -> A {
        if let Some(&number) = self.state.get(ident) {
            number
        } else {
            let default = A::top().into();
            self.state.insert(ident.clone(), default);
            default
        }
    }

    pub fn analyze_command(&mut self, c: &Command) -> &MemoryState<A> {
        match c {
            Command::Skip => (),
            Command::Seq(c1, c2) => {
                self.analyze_command(c1);
                self.analyze_command(c2);
            }
            Command::Assign(Var::Var(ident), sexpr) => {
                let a = self.interprete_sexpr(sexpr);
                self.state.insert(ident.clone(), a);
            }
            Command::Input(Var::Var(ident)) => {
                self.state.insert(ident.clone(), A::top().into());
            }
            Command::If(bexpr, c1, c2) => {
                let m1 = self
                    .clone()
                    .filter(bexpr)
                    .to_owned()
                    .analyze_command(c1)
                    .to_owned();

                self.filter(&bexpr.negate()).analyze_command(c2);

                self.join(&m1);
            }
            Command::While(bexpr, c) => {
                let mut i = 0;
                loop {
                    let mut prev_m = self.clone();
                    println!("prev m: {:?}", &prev_m);

                    prev_m.filter(bexpr);
                    println!("filtered: {:?}", &prev_m);

                    prev_m.analyze_command(c);
                    println!("analyzed: {:?}", &prev_m);

                    self.join(&prev_m);
                    println!("joined: {:?}", &self);

                    i += 1;
                    if prev_m.inclusion(self) || i == 10 {
                        break;
                    }
                }
                self.filter(&bexpr.negate());
                println!("negation filtered: {:?}", &self);
            }
        }
        self
    }

    fn join(&mut self, other: &MemoryState<A>) -> &mut Self {
        for (ident, a_other) in &other.state {
            self.state
                .entry(ident.clone())
                .and_modify(|a_self| *a_self = A::join(a_self, &a_other))
                .or_insert(a_other.clone());
        }

        self
    }

    fn filter(&mut self, bexpr: &BExpr) -> &mut Self {
        let ident = bexpr.get_ident();
        let a = self.get_from_state_or_insert_default(ident);

        if A::sat(&a, bexpr) {
            let a_filtered = A::refine(&a, bexpr);
            self.state.insert(ident.clone(), a_filtered);
        } else {
            self.set_all_vars_to_bottom();
        }

        self
    }

    fn inclusion(&self, other: &MemoryState<A>) -> bool {
        for (ident, a_other) in &other.state {
            if let Some(a_self) = self.state.get(ident) {
                if !A::inclusion(a_other, a_self) {
                    return false;
                }
            }
        }

        true
    }

    fn set_all_vars_to_bottom(&mut self) {
        for value in self.state.values_mut() {
            *value = A::bottom().into();
        }
    }
}
