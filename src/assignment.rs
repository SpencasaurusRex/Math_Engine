use super::*;

#[derive(Clone)]
pub struct Assignment {
    var: Variable,
    constant: Constant
}

impl Assignment {
    pub fn new(var: Variable, c: Constant) -> Assignment {
        Assignment {
            var: var,
            constant: c,
        }
    }

    pub fn variable(&self) -> &Variable {
        &self.var
    }

    pub fn constant(&self) -> &Constant {
        &self.constant
    }
}