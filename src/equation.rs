use super::*;

/// Two Expressions set equal
#[derive(Clone)]
pub struct Equation {
    left:  Expression,
    right: Expression,
}

impl Equation {
    pub fn new(left: Expression, right: Expression) -> Equation {
        Equation {
            left: left,
            right: right
        }
    }

    pub fn left(&self) -> &Expression {
        &self.left
    }
    
    pub fn right(&self) -> &Expression {
        &self.right
    }
}