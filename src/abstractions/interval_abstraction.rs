use std::ptr::eq;
use std::{cmp::Ordering, ops};

use crate::command_parser::{BExpr, Const};
use crate::interpreter::{AbstractProperties, Bottom, Top};

const EPS: f64 = 1e-5;

#[derive(Copy, Clone, Debug)]
pub struct Interval {
    pub a: f64,
    pub b: f64,
}

impl Interval {
    pub fn new(a: f64, b: f64) -> Interval {
        assert!(a < b);
        Interval { a, b }
    }
}

#[derive(Copy, Clone, Debug)]
enum IntervalAbstraction {
    Bottom,
    Interval(Interval),
    Top,
}

impl ops::Add for IntervalAbstraction {
    type Output = IntervalAbstraction;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Bottom => Self::Bottom,
            Self::Top => match rhs {
                Self::Bottom => Self::Bottom,
                _ => Self::Top,
            },
            Self::Interval(Interval {
                a: self_a,
                b: self_b,
            }) => match rhs {
                Self::Bottom => Self::Bottom,
                Self::Top => Self::Top,
                Self::Interval(Interval { a: rhs_a, b: rhs_b }) => {
                    let a = self_a + rhs_a;
                    let b = self_b + rhs_b;
                    IntervalAbstraction::Interval(Interval {
                        a: f64::min(a, b),
                        b: f64::max(a, b),
                    })
                }
            },
        }
    }
}

impl ops::Sub for IntervalAbstraction {
    type Output = IntervalAbstraction;
    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::Bottom => Self::Bottom,
            Self::Top => match rhs {
                Self::Bottom => Self::Bottom,
                _ => Self::Top,
            },
            Self::Interval(Interval {
                a: self_a,
                b: self_b,
            }) => match rhs {
                Self::Bottom => Self::Bottom,
                Self::Top => Self::Top,
                Self::Interval(Interval { a: rhs_a, b: rhs_b }) => {
                    let a = self_a - rhs_a;
                    let b = self_b - rhs_b;
                    IntervalAbstraction::Interval(Interval {
                        a: f64::min(a, b),
                        b: f64::max(a, b),
                    })
                }
            },
        }
    }
}

impl ops::Mul for IntervalAbstraction {
    type Output = IntervalAbstraction;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::Bottom => Self::Bottom,
            Self::Top => match rhs {
                Self::Bottom => Self::Bottom,
                _ => Self::Top,
            },
            Self::Interval(Interval {
                a: self_a,
                b: self_b,
            }) => match rhs {
                Self::Bottom => Self::Bottom,
                Self::Top => Self::Top,
                Self::Interval(Interval { a: rhs_a, b: rhs_b }) => {
                    let a = self_a * rhs_a;
                    let b = self_b * rhs_b;
                    IntervalAbstraction::Interval(Interval {
                        a: f64::min(a, b),
                        b: f64::max(a, b),
                    })
                }
            },
        }
    }
}

impl ops::Div for IntervalAbstraction {
    type Output = IntervalAbstraction;
    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Self::Bottom => Self::Bottom,
            Self::Top => match rhs {
                Self::Bottom => Self::Bottom,
                _ => Self::Top,
            },
            Self::Interval(Interval {
                a: self_a,
                b: self_b,
            }) => match rhs {
                Self::Bottom => Self::Bottom,
                Self::Top => Self::Top,
                Self::Interval(Interval { a: rhs_a, b: rhs_b }) => {
                    let a = self_a / rhs_a;
                    let b = self_b / rhs_b;
                    IntervalAbstraction::Interval(Interval {
                        a: f64::min(a, b),
                        b: f64::max(a, b),
                    })
                }
            },
        }
    }
}

impl ops::Neg for IntervalAbstraction {
    type Output = IntervalAbstraction;

    fn neg(self) -> Self::Output {
        match self {
            Self::Bottom => Self::Top,
            Self::Top => Self::Bottom,
            Self::Interval(Interval { a, b }) => {
                let a = -a;
                let b = -b;
                IntervalAbstraction::Interval(Interval {
                    a: f64::min(a, b),
                    b: f64::max(a, b),
                })
            }
        }
    }
}

impl From<f64> for IntervalAbstraction {
    fn from(value: f64) -> Self {
        Self::Interval(Interval { a: value, b: value })
    }
}

impl PartialEq for IntervalAbstraction {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Bottom => match other {
                Self::Bottom => true,
                _ => false,
            },
            Self::Interval(Interval {
                a: a_self,
                b: b_self,
            }) => match other {
                Self::Bottom => false,
                Self::Interval(Interval {
                    a: a_other,
                    b: b_other,
                }) => (a_self - a_other).abs() < EPS && (b_self - b_other).abs() < EPS,
                Self::Top => false,
            },
            Self::Top => match other {
                Self::Top => true,
                _ => false,
            },
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !eq(self, other)
    }
}

impl PartialOrd for IntervalAbstraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Self::Bottom => match other {
                Self::Bottom => Some(Ordering::Equal),
                _ => Some(Ordering::Less),
            },
            Self::Interval(Interval {
                a: a_self,
                b: b_self,
            }) => match other {
                Self::Bottom => Some(Ordering::Greater),
                Self::Interval(Interval {
                    a: a_other,
                    b: b_other,
                }) => {
                    if a_self == a_other && b_self == b_other {
                        Some(Ordering::Equal)
                    } else if b_self - EPS / 2.0 < a_other + EPS / 2.0 {
                        Some(Ordering::Less)
                    } else if b_other - EPS / 2.0 < a_self + EPS / 2.0 {
                        Some(Ordering::Greater)
                    } else {
                        None
                    }
                }
                Self::Top => Some(Ordering::Less),
            },
            Self::Top => match other {
                Self::Top => Some(Ordering::Equal),
                _ => Some(Ordering::Greater),
            },
        }
    }
}

impl From<Top> for IntervalAbstraction {
    fn from(_: Top) -> Self {
        Self::Top
    }
}

impl From<Bottom> for IntervalAbstraction {
    fn from(_: Bottom) -> Self {
        Self::Bottom
    }
}

impl AbstractProperties<IntervalAbstraction> for IntervalAbstraction {
    fn bottom() -> Bottom {
        Bottom
    }

    fn top() -> Top {
        Top
    }

    fn sat(a: &IntervalAbstraction, bexpr: &BExpr) -> bool {
        match *a {
            Self::Bottom => false,
            Self::Top => true,
            Self::Interval(Interval { a, b }) => match bexpr {
                BExpr::EQ(_, Const::Const(number)) => a <= *number && *number <= b,
                BExpr::GE(_, Const::Const(number)) => *number <= b,
                BExpr::GT(_, Const::Const(number)) => *number < b,
                BExpr::LE(_, Const::Const(number)) => a <= *number,
                BExpr::LT(_, Const::Const(number)) => a < *number,
                BExpr::NE(_, Const::Const(number)) => *number < a || b < *number,
            },
        }
    }

    fn inclusion(a0: IntervalAbstraction, a1: IntervalAbstraction) -> bool {
        match a0 {
            Self::Bottom => true,
            Self::Top => match a1 {
                Self::Top => true,
                _ => false,
            },
            Self::Interval(Interval { a: a0_a, b: a0_b }) => match a1 {
                Self::Bottom => false,
                Self::Top => true,
                Self::Interval(Interval { a: a1_a, b: a1_b }) => a1_a <= a0_a && a0_b <= a1_b,
            },
        }
    }

    fn join(a0: &IntervalAbstraction, a1: &IntervalAbstraction) -> IntervalAbstraction {
        match a0 {
            Self::Bottom => *a1,
            Self::Top => Self::Top,
            Self::Interval(Interval { a: a0_a, b: a0_b }) => match a1 {
                Self::Bottom => *a0,
                Self::Top => Self::Top,
                Self::Interval(Interval { a: a1_a, b: a1_b }) => {
                    let a = f64::min(*a0_a, *a1_a);
                    let b = f64::max(*a0_b, *a1_b);

                    if a == f64::MIN && b == f64::MAX {
                        Self::Top
                    } else {
                        Self::Interval(Interval { a, b })
                    }
                }
            },
        }
    }

    fn refine(a: &IntervalAbstraction, bexpr: &BExpr) -> IntervalAbstraction {
        if !Self::sat(a, bexpr) {
            return Self::Bottom;
        }

        match a {
            Self::Bottom => Self::Bottom,
            Self::Top => match bexpr {
                BExpr::EQ(_, Const::Const(number)) => {
                    let number = *number;
                    number.into()
                }
                BExpr::LE(_, Const::Const(number)) => Self::Interval(Interval {
                    a: f64::MIN,
                    b: *number,
                }),
                BExpr::LT(_, Const::Const(number)) => Self::Interval(Interval {
                    a: f64::MIN,
                    b: *number - EPS,
                }),

                BExpr::GE(_, Const::Const(number)) => Self::Interval(Interval {
                    a: *number,
                    b: f64::MAX,
                }),
                BExpr::GT(_, Const::Const(number)) => Self::Interval(Interval {
                    a: *number + EPS,
                    b: f64::MAX,
                }),
                BExpr::NE(_, Const::Const(_)) => Self::Bottom,
            },
            Self::Interval(Interval { a, b }) => match bexpr {
                BExpr::EQ(_, Const::Const(number)) => {
                    let number = *number;
                    number.into()
                }
                BExpr::LE(_, Const::Const(number)) => {
                    Self::Interval(Interval { a: *a, b: *number })
                }
                BExpr::LT(_, Const::Const(number)) => Self::Interval(Interval {
                    a: *a,
                    b: *number - EPS,
                }),

                BExpr::GE(_, Const::Const(number)) => {
                    Self::Interval(Interval { a: *number, b: *b })
                }
                BExpr::GT(_, Const::Const(number)) => Self::Interval(Interval {
                    a: *number + EPS,
                    b: *b,
                }),
                BExpr::NE(_, Const::Const(number)) => {
                    if b < number {
                        Self::Interval(Interval {
                            a: *a,
                            b: *number - EPS,
                        })
                    } else {
                        Self::Interval(Interval {
                            a: *number + EPS,
                            b: *b,
                        })
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command_parser::parse;
    use crate::MemoryState;
    use std::collections::HashMap;

    #[test]
    fn example_3_13() {
        let program = "if (x > 7) {y := x - 7} else {y := 7 - x}";
        let command = parse(&program);

        let mut pre: MemoryState<IntervalAbstraction> = MemoryState::new();
        let post_analyzed = pre.analyze_command(&command);

        let post_truth = MemoryState::from_state(HashMap::from([
            ("x".to_string(), IntervalAbstraction::Top),
            (
                "y".to_string(),
                IntervalAbstraction::Interval(Interval {
                    a: 0.0,
                    b: f64::MAX,
                }),
            ),
        ]));
        assert_eq!(post_truth, *post_analyzed);
    }
}
