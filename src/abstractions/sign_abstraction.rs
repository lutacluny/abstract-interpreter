use std::{cmp::Ordering, ops};

use crate::command_parser::{BExpr, Const};
use crate::interpreter::{AbstractProperties, Bottom, Top};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum SignAbstraction {
    Bottom,
    Neg,
    Zero,
    Pos,
    Top,
}

impl ops::Add for SignAbstraction {
    type Output = SignAbstraction;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Bottom => Self::Bottom,
            Self::Neg => match rhs {
                Self::Bottom => Self::Bottom,
                Self::Neg => Self::Neg,
                Self::Zero => self,
                _ => Self::Top,
            },
            Self::Pos => match rhs {
                Self::Bottom => Self::Bottom,
                Self::Pos => Self::Pos,
                Self::Zero => self,
                _ => Self::Top,
            },
            Self::Top => match rhs {
                Self::Bottom => Self::Bottom,
                _ => Self::Top,
            },
            Self::Zero => rhs,
        }
    }
}

impl ops::Neg for SignAbstraction {
    type Output = SignAbstraction;

    fn neg(self) -> Self::Output {
        match self {
            Self::Bottom => Self::Bottom,
            Self::Neg => Self::Pos,
            Self::Pos => Self::Neg,
            Self::Top => Self::Top,
            Self::Zero => Self::Zero,
        }
    }
}

impl ops::Sub for SignAbstraction {
    type Output = SignAbstraction;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::Bottom => Self::Bottom,
            Self::Neg => match rhs {
                Self::Bottom => Self::Bottom,
                Self::Pos => Self::Neg,
                Self::Zero => self,
                _ => Self::Top,
            },
            Self::Pos => match rhs {
                Self::Bottom => Self::Bottom,
                Self::Neg => Self::Pos,
                Self::Zero => self,
                _ => Self::Top,
            },
            Self::Top => match rhs {
                Self::Bottom => Self::Bottom,
                _ => Self::Top,
            },
            Self::Zero => rhs,
        }
    }
}

impl ops::Mul for SignAbstraction {
    type Output = SignAbstraction;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::Bottom => Self::Bottom,
            Self::Neg => match rhs {
                Self::Bottom => Self::Bottom,
                Self::Neg => Self::Pos,
                Self::Pos => Self::Neg,
                Self::Top => Self::Top,
                Self::Zero => Self::Zero,
            },
            Self::Pos => match rhs {
                Self::Bottom => Self::Bottom,
                Self::Neg => Self::Neg,
                Self::Pos => Self::Pos,
                Self::Top => Self::Top,
                Self::Zero => Self::Zero,
            },
            Self::Top => match rhs {
                Self::Bottom => Self::Bottom,
                Self::Zero => Self::Zero,
                _ => Self::Top,
            },
            Self::Zero => Self::Zero,
        }
    }
}

impl ops::Div for SignAbstraction {
    type Output = SignAbstraction;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs
    }
}

impl From<f64> for SignAbstraction {
    fn from(number: f64) -> Self {
        if number == 0.0 {
            Self::Zero
        } else if number > 0.0 {
            Self::Pos
        } else {
            Self::Neg
        }
    }
}

impl PartialOrd for SignAbstraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Self::Bottom => match other {
                Self::Bottom => Some(Ordering::Equal),
                _ => Some(Ordering::Less),
            },
            Self::Neg => match other {
                Self::Bottom => Some(Ordering::Greater),
                Self::Neg => Some(Ordering::Equal),
                Self::Zero => Some(Ordering::Less),
                Self::Pos => Some(Ordering::Less),
                Self::Top => Some(Ordering::Less),
            },
            Self::Pos => match other {
                Self::Bottom => Some(Ordering::Greater),
                Self::Neg => Some(Ordering::Greater),
                Self::Zero => Some(Ordering::Greater),
                Self::Pos => Some(Ordering::Equal),
                Self::Top => Some(Ordering::Less),
            },
            Self::Top => match other {
                Self::Top => Some(Ordering::Equal),
                _ => Some(Ordering::Greater),
            },
            Self::Zero => match other {
                Self::Bottom => Some(Ordering::Greater),
                Self::Neg => Some(Ordering::Greater),
                Self::Zero => Some(Ordering::Equal),
                Self::Pos => Some(Ordering::Less),
                Self::Top => Some(Ordering::Less),
            },
        }
    }
}
impl AbstractProperties<SignAbstraction> for SignAbstraction {
    fn bottom() -> Bottom {
        Bottom
    }

    fn top() -> Top {
        Top
    }

    fn sat(a: &SignAbstraction, bexpr: &BExpr) -> bool {
        if *a == SignAbstraction::Bottom {
            return false;
        }

        let a_number: SignAbstraction;

        match bexpr {
            BExpr::EQ(_, Const::Const(number)) => {
                let number = *number;
                a_number = number.into();
            }
            BExpr::GE(_, Const::Const(number)) => {
                let number = *number;
                a_number = number.into();
            }
            BExpr::GT(_, Const::Const(number)) => {
                let number = *number;
                if number == 0.0 {
                    a_number = Self::Pos;
                } else {
                    a_number = number.into();
                }
            }
            BExpr::LE(_, Const::Const(number)) => {
                let number = *number;
                a_number = number.into();
            }
            BExpr::LT(_, Const::Const(number)) => {
                let number = *number;
                if number == 0.0 {
                    a_number = Self::Neg;
                } else {
                    a_number = number.into();
                }
            }
            BExpr::NE(_, Const::Const(number)) => {
                let number = *number;
                a_number = number.into();
                return !Self::inclusion(a_number, *a);
            }
        }

        Self::inclusion(a_number, *a)
    }
    fn inclusion(a0: Self, a1: Self) -> bool {
        match a0 {
            Self::Bottom => true,

            Self::Neg => match a1 {
                Self::Bottom => false,
                Self::Neg => true,
                Self::Zero => false,
                Self::Pos => false,
                Self::Top => true,
            },
            Self::Zero => match a1 {
                Self::Bottom => false,
                Self::Neg => true,
                Self::Zero => true,
                Self::Pos => true,
                Self::Top => true,
            },
            Self::Pos => match a1 {
                Self::Bottom => false,
                Self::Neg => false,
                Self::Zero => false,
                Self::Pos => true,
                Self::Top => true,
            },
            Self::Top => match a1 {
                Self::Top => true,
                _ => false,
            },
        }
    }

    fn join(a0: &Self, a1: &Self) -> Self {
        match a0 {
            Self::Bottom => *a1,
            Self::Neg => match a1 {
                Self::Bottom => *a0,
                Self::Neg => *a0,
                Self::Zero => *a0,
                Self::Pos => Self::Top,
                Self::Top => Self::Top,
            },
            Self::Zero => match a1 {
                Self::Bottom => *a0,
                Self::Neg => Self::Neg,
                Self::Zero => *a0,
                Self::Pos => Self::Pos,
                Self::Top => Self::Top,
            },
            Self::Pos => match a1 {
                Self::Bottom => *a0,
                Self::Neg => Self::Top,
                Self::Zero => *a0,
                Self::Pos => Self::Pos,
                Self::Top => Self::Top,
            },
            Self::Top => Self::Top,
        }
    }

    fn refine(a: &Self, bexpr: &BExpr) -> Self {
        if !Self::sat(a, bexpr) {
            return Self::Bottom;
        }
        match bexpr {
            BExpr::EQ(_, Const::Const(number)) => {
                let number = *number;
                number.into()
            }
            BExpr::LE(_, Const::Const(number)) => {
                if *number <= 0.0 {
                    Self::Neg
                } else {
                    Self::Top
                }
            }
            BExpr::LT(_, Const::Const(number)) => {
                if *number < 0.0 {
                    Self::Neg
                } else {
                    Self::Top
                }
            }
            BExpr::GE(_, Const::Const(number)) => {
                if *number >= 0.0 {
                    Self::Pos
                } else {
                    Self::Top
                }
            }
            BExpr::GT(_, Const::Const(number)) => {
                if *number > 0.0 {
                    Self::Pos
                } else {
                    Self::Top
                }
            }
            BExpr::NE(_, Const::Const(_)) => Self::Bottom,
        }
    }
}

impl From<Top> for SignAbstraction {
    fn from(_: Top) -> Self {
        Self::Top
    }
}

impl From<Bottom> for SignAbstraction {
    fn from(_: Bottom) -> Self {
        Self::Bottom
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command_parser::parse;
    use crate::MemoryState;
    use std::collections::HashMap;

    #[test]
    fn skip() {
        let program = "skip";
        let command = parse(&program);

        let mut pre: MemoryState<SignAbstraction> = MemoryState::new();
        let post_analyzed = pre.analyze_command(&command);

        let post_truth = MemoryState::from_state(HashMap::from([]));
        assert_eq!(post_truth, *post_analyzed);
    }

    #[test]
    fn assign() {
        let program = "x := 50";
        let command = parse(&program);

        let mut pre: MemoryState<SignAbstraction> = MemoryState::new();
        let post_analyzed = pre.analyze_command(&command);

        let post_truth =
            MemoryState::from_state(HashMap::from([("x".to_string(), SignAbstraction::Pos)]));
        assert_eq!(post_truth, *post_analyzed);
    }

    #[test]
    fn input() {
        let program = "input(x)";
        let command = parse(&program);

        let mut pre: MemoryState<SignAbstraction> = MemoryState::new();
        let post_analyzed = pre.analyze_command(&command);

        let post_truth =
            MemoryState::from_state(HashMap::from([("x".to_string(), SignAbstraction::Top)]));
        assert_eq!(post_truth, *post_analyzed);
    }

    #[test]
    fn cif() {
        let program = "if (x < 0) {y := x} else {skip}";
        let command = parse(&program);

        let mut pre: MemoryState<SignAbstraction> = MemoryState::new();
        let post_analyzed = pre.analyze_command(&command);

        let post_truth = MemoryState::from_state(HashMap::from([
            ("x".to_string(), SignAbstraction::Top),
            ("y".to_string(), SignAbstraction::Top),
        ]));
        assert_eq!(post_truth, *post_analyzed);
    }

    #[test]
    fn figure_3_9_a_with_pre_condition() {
        let program = "x := 0; while (x >= 0) {x := x + 1}";
        let command = parse(&program);

        let mut pre: MemoryState<SignAbstraction> =
            MemoryState::from_state(HashMap::from([("x".to_string(), SignAbstraction::Pos)]));
        let post_analyzed = pre.analyze_command(&command);

        let post_truth =
            MemoryState::from_state(HashMap::from([("x".to_string(), SignAbstraction::Bottom)]));
        assert_eq!(post_truth, *post_analyzed);
    }

    #[test]
    fn seq() {
        let program = "skip;skip";
        let command = parse(&program);

        let mut pre: MemoryState<SignAbstraction> = MemoryState::new();
        let post_analyzed = pre.analyze_command(&command);

        let post_truth = MemoryState::from_state(HashMap::from([]));
        assert_eq!(post_truth, *post_analyzed);
    }
}
