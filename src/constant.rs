use super::*;

/// A value without an associated variable
#[derive(Clone)]
pub enum Constant {
    E,
    Pi,
    Decimal(f64),
    Int(i32),
    // TODO add fractional
}

impl Constant {
    pub fn as_expression(&self) -> Expression {
        Expression::Constant(self.clone())
    }
    
    pub fn to_expression(self) -> Expression {
        Expression::Constant(self)
    }
}

impl Evaluate for Constant {
    fn evaluate_f64(&self, _: &Vec<Assignment>) -> Result<f64,String> {
        println!("Evaluating {}", self);
        match *self {
            Constant::E => Ok(std::f64::consts::E),
            Constant::Pi => Ok(std::f64::consts::PI),
            Constant::Decimal(x) => Ok(x),
            Constant::Int(x) => Ok(x as f64),
        }
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Constant::E => write!(f, "e"),
            Constant::Pi => write!(f, "pi"),
            Constant::Decimal(x) => write!(f, "{}", x.to_string()),
            Constant::Int(x) => write!(f, "{}", x.to_string())
        }
    }
}
