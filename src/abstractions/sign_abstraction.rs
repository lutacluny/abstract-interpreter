use std::{cmp::Ordering, ops};

use crate::command_parser::{BExpr, Const, Var};
use crate::interpreter::{AbstractProperties, Bottom, Top};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum SignAbstraction {
    Bottom,
    LE0,
    GE0,
    Top,
}

impl ops::Add for SignAbstraction {
    type Output = SignAbstraction;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Bottom => Self::Bottom,
            Self::LE0 => match rhs {
                Self::Bottom => Self::Bottom,
                Self::LE0 => Self::LE0,
                _ => Self::Top,
            },
            Self::GE0 => match rhs {
                Self::Bottom => Self::Bottom,
                Self::GE0 => Self::GE0,
                _ => Self::Top,
            },
            Self::Top => match rhs {
                Self::Bottom => Self::Bottom,
                _ => Self::Top,
            },
        }
    }
}

impl ops::Neg for SignAbstraction {
    type Output = SignAbstraction;

    fn neg(self) -> Self::Output {
        match self {
            Self::Bottom => Self::Bottom,
            Self::LE0 => Self::GE0,
            Self::GE0 => Self::LE0,
            Self::Top => Self::Top,
        }
    }
}

impl ops::Sub for SignAbstraction {
    type Output = SignAbstraction;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::Bottom => Self::Bottom,
            Self::LE0 => match rhs {
                Self::Bottom => Self::Bottom,
                Self::GE0 => Self::LE0,
                _ => Self::Top,
            },
            Self::GE0 => match rhs {
                Self::Bottom => Self::Bottom,
                Self::LE0 => Self::GE0,
                _ => Self::Top,
            },
            Self::Top => match rhs {
                Self::Bottom => Self::Bottom,
                _ => Self::Top,
            },
        }
    }
}

impl ops::Mul for SignAbstraction {
    type Output = SignAbstraction;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::Bottom => Self::Bottom,
            Self::LE0 => match rhs {
                Self::Bottom => Self::Bottom,
                Self::LE0 => Self::GE0,
                Self::GE0 => Self::LE0,
                Self::Top => Self::Top,
            },
            Self::GE0 => match rhs {
                Self::Bottom => Self::Bottom,
                Self::LE0 => Self::LE0,
                Self::GE0 => Self::GE0,
                Self::Top => Self::Top,
            },
            Self::Top => match rhs {
                Self::Bottom => Self::Bottom,
                _ => Self::Top,
            },
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
        if number <= 0.0 {
            Self::LE0
        } else {
            Self::GE0
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
            Self::LE0 => match other {
                Self::Bottom => Some(Ordering::Greater),
                Self::LE0 => Some(Ordering::Equal),
                Self::GE0 => None,
                Self::Top => Some(Ordering::Less),
            },
            Self::GE0 => match other {
                Self::Bottom => Some(Ordering::Greater),
                Self::LE0 => None,
                Self::GE0 => Some(Ordering::Equal),
                Self::Top => Some(Ordering::Less),
            },
            Self::Top => match other {
                Self::Top => Some(Ordering::Equal),
                _ => Some(Ordering::Less),
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

    fn inclusion(a0: Self, a1: Self) -> bool {
        a0 == Self::Bottom || a1 == Self::Top || a0 == a1
    }

    fn join(a0: &Self, a1: &Self) -> Self {
        if let Some(order) = Self::partial_cmp(&a0, &a1) {
            match order {
                Ordering::Equal => *a0,
                Ordering::Less => *a1,
                Ordering::Greater => *a0,
            }
        } else {
            Self::Top
        }
    }

    fn filter(a: &Self, bexpr: &BExpr) -> Self {
        match bexpr {
            BExpr::EQ(_, Const::Const(number)) => {
                if *a == number.clone().into() {
                    *a
                } else {
                    Self::Bottom
                }
            }
            BExpr::GE(_, Const::Const(number)) => {
                let number = *number;
                let a_number = number.into();

                if *a == a_number {
                    *a
                } else if *a > a_number {
                    a_number
                } else {
                    Self::Bottom
                }
            }
            BExpr::GT(_, Const::Const(number)) => {
                let number = *number;
                let a_number = number.into();

                if *a > a_number {
                    a_number
                } else {
                    Self::Bottom
                }
            }
            BExpr::LE(_, Const::Const(number)) => {
                let number = *number;
                let a_number = number.into();

                if *a == a_number {
                    *a
                } else if *a < a_number {
                    a_number
                } else {
                    Self::Bottom
                }
            }
            BExpr::LT(_, Const::Const(number)) => {
                let number = *number;
                let a_number = number.into();

                if *a < a_number {
                    a_number
                } else {
                    Self::Bottom
                }
            }
            BExpr::NE(_, Const::Const(number)) => {
                let number = *number;
                let a_number = number.into();

                if *a != a_number {
                    a_number
                } else {
                    Self::Bottom
                }
            }
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
