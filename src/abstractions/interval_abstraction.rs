use core::f64;
use std::{cmp::Ordering, ops};

use crate::command_parser::{BExpr, Const};
use crate::interpreter::{AbstractProperties, Bottom, Params, Top};

const EPS: f64 = 1e-5;

#[derive(Copy, Clone, Debug)]
pub struct Interval {
    pub a: f64,
    pub b: f64,
}

impl Interval {
    pub fn new(a: f64, b: f64) -> Interval {
        assert!(a <= b);
        Interval { a, b }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum IntervalAbstraction {
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

                    (f64::min(a, b), f64::max(a, b)).into()
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

                    (f64::min(a, b), f64::max(a, b)).into()
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

                    (f64::min(a, b), f64::max(a, b)).into()
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
                    (f64::min(a, b), f64::max(a, b)).into()
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
                (f64::min(a, b), f64::max(a, b)).into()
            }
        }
    }
}

impl From<f64> for IntervalAbstraction {
    fn from(value: f64) -> Self {
        (value, value).into()
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
        !IntervalAbstraction::eq(self, other)
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

    fn first_includes_second(a0: &IntervalAbstraction, a1: &IntervalAbstraction) -> bool {
        match a0 {
            Self::Bottom => match a1 {
                Self::Bottom => true,
                _ => false,
            },
            Self::Top => true,
            Self::Interval(Interval { a: a0_a, b: a0_b }) => match a1 {
                Self::Bottom => true,
                Self::Top => false,
                Self::Interval(Interval { a: a1_a, b: a1_b }) => *a0_a <= *a1_a && *a1_b <= *a0_b,
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
                        (a, b).into()
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
                BExpr::LE(_, Const::Const(number)) => (f64::MIN, *number).into(),
                BExpr::LT(_, Const::Const(number)) => (f64::MIN, *number - EPS).into(),
                BExpr::GE(_, Const::Const(number)) => (*number, f64::MAX).into(),
                BExpr::GT(_, Const::Const(number)) => (*number + EPS, f64::MAX).into(),
                BExpr::NE(_, Const::Const(_)) => Self::Bottom,
            },
            Self::Interval(Interval { a, b }) => match bexpr {
                BExpr::EQ(_, Const::Const(number)) => {
                    let number = *number;
                    number.into()
                }
                BExpr::LE(_, Const::Const(number)) => {
                    if *number <= *b {
                        (*a, *number).into()
                    } else {
                        (*a, *b).into()
                    }
                }
                BExpr::LT(_, Const::Const(number)) => {
                    if *number < *b {
                        (*a, *number).into()
                    } else {
                        (*a, *b).into()
                    }
                }

                BExpr::GE(_, Const::Const(number)) => {
                    if *number >= *a {
                        (*number, *b).into()
                    } else {
                        (*a, *b).into()
                    }
                }
                BExpr::GT(_, Const::Const(number)) => {
                    if *number > *a {
                        (*number, *b).into()
                    } else {
                        (*a, *b).into()
                    }
                }
                BExpr::NE(_, Const::Const(_)) => Self::Bottom,
            },
        }
    }

    fn widen(
        a0: &IntervalAbstraction,
        a1: &IntervalAbstraction,
        widening_treshold: &IntervalAbstraction,
    ) -> IntervalAbstraction {
        match a0 {
            IntervalAbstraction::Bottom => IntervalAbstraction::Top,
            IntervalAbstraction::Top => IntervalAbstraction::Top,
            IntervalAbstraction::Interval(Interval { a: a0_a, b: a0_b }) => match a1 {
                IntervalAbstraction::Bottom => *a0,
                IntervalAbstraction::Top => IntervalAbstraction::Top,
                IntervalAbstraction::Interval(Interval { a: a1_a, b: a1_b }) => {
                    if *a1_a < *a0_a {
                        if *a0_b < *a1_b {
                            IntervalAbstraction::Top
                        } else {
                            match widening_treshold {
                                IntervalAbstraction::Top => (f64::MIN, *a0_b).into(),
                                IntervalAbstraction::Bottom => *a0,
                                IntervalAbstraction::Interval(Interval { a: t_a, b: _ }) => {
                                    (*t_a, *a0_b).into()
                                }
                            }
                        }
                    } else {
                        if *a0_b < *a1_b {
                            match widening_treshold {
                                IntervalAbstraction::Top => (*a0_a, f64::MAX).into(),
                                IntervalAbstraction::Bottom => *a0,
                                IntervalAbstraction::Interval(Interval { a: _, b: t_b }) => {
                                    (*a0_a, *t_b).into()
                                }
                            }
                        } else {
                            *a0
                        }
                    }
                }
            },
        }
    }
}

impl From<(f64, f64)> for IntervalAbstraction {
    fn from((a, b): (f64, f64)) -> Self {
        IntervalAbstraction::Interval(Interval::new(a, b))
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
        let post_analyzed = pre.analyze_command(&command, &Params::no_widening());

        let post_truth = MemoryState::from_state(HashMap::from([
            ("x".to_string(), IntervalAbstraction::Top),
            ("y".to_string(), (0.0, f64::MAX).into()),
        ]));
        assert_eq!(post_truth, *post_analyzed);
    }

    #[test]
    fn figure_5_4_without_unrolling_with_x_initialized_before() {
        let program = "i := 1; while (i > 0) {if (x < 0) {x := 0} else {x := 1 + x}; if (x > 1000) {x := 0} else {x := x + 1}; input(i)}";
        let command = parse(&program);

        let mut pre: MemoryState<IntervalAbstraction> =
            MemoryState::from_state(HashMap::from([("x".to_string(), IntervalAbstraction::Top)]));

        let post_analyzed = pre
            .analyze_command(&command, &Params::no_widening())
            .to_owned();

        let x_analyzed = post_analyzed.lookup_var("x").unwrap();
        let x_truth = IntervalAbstraction::Top;
        assert_eq!(x_truth, *x_analyzed);
    }

    #[test]
    fn figure_5_4_with_unrolling_with_x_initialized_in_the_loop_body_default_behavior() {
        let program = "i := 1; while (i > 0) {if (x < 0) {x := 0} else {x := 1 + x}; if (x > 1000) {x := 0} else {x := x + 1}; input(i)}";
        let command = parse(&program);

        let mut pre: MemoryState<IntervalAbstraction> = MemoryState::new();

        let post_analyzed = pre
            .analyze_command(&command, &Params::no_widening())
            .to_owned();

        let x_analyzed = post_analyzed.lookup_var("x").unwrap();
        let x_truth = IntervalAbstraction::Interval(Interval { a: 0.0, b: 1001.0 });
        assert_eq!(x_truth, *x_analyzed);
    }

    #[test]
    fn figure_5_4_with_unrolling() {
        let program = "i := 1; while (i > 0) {if (x < 0) {x := 0} else {x := 1 + x}; if (x > 1000) {x := 0} else {x := x + 1}; input(i)}";
        let command = parse(&program);

        let mut pre: MemoryState<IntervalAbstraction> = MemoryState::new();

        let params = Params {
            loop_unrollings: 1,
            use_widening: false,
            widening_delays: 0,
            widening_treshold: IntervalAbstraction::Top,
        };

        let post_analyzed = pre.analyze_command(&command, &params);
        let x_analyzed = post_analyzed.lookup_var("x").unwrap();

        let x_truth = IntervalAbstraction::Interval(Interval { a: 0.0, b: 1001.0 });
        assert_eq!(x_truth, *x_analyzed);
    }

    #[test]
    fn figure_5_5_b_without_widening() {
        let program = "x := 0; while (x <= 100) {if (x >= 50) {x := 10} else {x := x + 1}}";

        let command = parse(&program);

        let mut pre: MemoryState<IntervalAbstraction> = MemoryState::new();

        let post_analyzed = pre.analyze_command(&command, &Params::no_widening());
        let x_analyzed = post_analyzed.lookup_var("x").unwrap();

        let x_truth: IntervalAbstraction = IntervalAbstraction::Bottom;
        assert_eq!(x_truth, *x_analyzed);
    }

    #[test]
    fn figure_5_5_b_with_widening() {
        let program = "x := 0; while (x <= 100) {if (x >= 50) {x := 10} else {x := x + 1}}";

        let command = parse(&program);

        let mut pre: MemoryState<IntervalAbstraction> = MemoryState::new();

        let params = Params {
            use_widening: true,
            loop_unrollings: 0,
            widening_delays: 0,
            widening_treshold: IntervalAbstraction::Top,
        };

        let post_analyzed = pre.analyze_command(&command, &params);
        let x_analyzed = post_analyzed.lookup_var("x").unwrap();

        let x_truth: IntervalAbstraction = (100.0, f64::MAX).into();
        assert_eq!(x_truth, *x_analyzed);
    }

    #[test]
    fn figure_5_5_b_with_delayed_widening() {
        let program = "x := 0; while (x <= 100) {if (x >= 50) {x := 10} else {x := 1 + x}}";

        let command = parse(&program);

        let mut pre: MemoryState<IntervalAbstraction> = MemoryState::new();

        let params = Params {
            use_widening: true,
            loop_unrollings: 0,
            widening_delays: 51,
            widening_treshold: IntervalAbstraction::Top,
        };

        let post_analyzed = pre.analyze_command(&command, &params);
        let x_analyzed = post_analyzed.lookup_var("x").unwrap();

        let x_truth = IntervalAbstraction::Bottom;
        assert_eq!(x_truth, *x_analyzed);
    }

    #[test]
    fn figure_5_5_b_with_widening_treshold() {
        let program = "x := 0; while (x <= 100) {if (x >= 50) {x := 10} else {x := x + 1}}";

        let command = parse(&program);

        let mut pre: MemoryState<IntervalAbstraction> = MemoryState::new();

        let params = Params {
            use_widening: true,
            loop_unrollings: 0,
            widening_delays: 0,
            widening_treshold: (-50.0, 50.0).into(),
        };

        let post_analyzed = pre.analyze_command(&command, &params);
        let x_analyzed = post_analyzed.lookup_var("x").unwrap();

        let x_truth = IntervalAbstraction::Bottom;
        assert_eq!(x_truth, *x_analyzed);
    }

    #[test]
    fn figure_5_2_coalescent_product_domain() {
        let program = "x := 8; y := 1; if (x < 0) {y := 0} else {skip}";
        let command = parse(&program);

        let mut pre: MemoryState<IntervalAbstraction> = MemoryState::new();
        let post_analyzed = pre.analyze_command(&command, &Params::no_widening());

        let post_truth = MemoryState::from_state(HashMap::from([
            ("x".to_string(), (8.0, 8.0).into()),
            ("y".to_string(), (1.0, 1.0).into()),
        ]));
        assert_eq!(post_truth, *post_analyzed);
    }
}
