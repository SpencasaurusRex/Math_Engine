use super::*;

/// Two Expressions set equal
#[derive(Clone)]
pub struct Equation {
    pub left:  Expression,
    pub right: Expression,
}
