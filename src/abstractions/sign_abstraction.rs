use std::ops;

use crate::interpreter::{Bottom, HasBottom, HasTop, Top};

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
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

impl HasTop for SignAbstraction {
    fn top() -> Top {
        Top
    }
}

impl From<Top> for SignAbstraction {
    fn from(_: Top) -> Self {
        Self::Top
    }
}

impl HasBottom for SignAbstraction {
    fn bottom() -> Bottom {
        Bottom
    }
}

impl From<Bottom> for SignAbstraction {
    fn from(_: Bottom) -> Self {
        Self::Bottom
    }
}
