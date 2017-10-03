use super::*;

/// An expression to a power (ex: x^2)
#[derive(Clone)]
pub struct BasicTerm {
    base: Box<Expression>,
    power: Box<Expression>
}

impl BasicTerm {
    // TODO: Change this and other constructors to take references? (Will have to deal with lifetimes)
    pub fn new<T: Into<Expression>, U: Into<Expression>>(base: T, pow: U) -> BasicTerm {
        BasicTerm {
            base: Box::new(base.into()),
            power: Box::new(pow.into()),
        }
    }
}

impl From<Expression> for BasicTerm {
    fn from(exp: Expression) -> BasicTerm {
        BasicTerm {
            base: Box::new(exp.into()),
            power: Box::new(Expression::Constant(Constant::Int(1))),
        }
    }
}

impl fmt::Display for BasicTerm {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.power {
            Expression::Constant(Constant::Int(1)) => {
                write!(f, "{}", *self.base)
            }
            _ => write!(f, "({})^({})", *self.base, *self.power)
        }
    }
}

impl Evaluate for BasicTerm {
    fn evaluate_f64(&self, a: &Vec<Assignment>) -> Result<f64,String> {
        println!("Evaluating basic term: {}", self);
        // TODO special cases (ex. power = 0, base =/= 0, ans = 1)
        let base_eval = self.base.evaluate_f64(a);
        let power_eval = self.power.evaluate_f64(a);
        match (base_eval, power_eval) {
            (Ok(base), Ok(power)) => Ok(base.powf(power)),
            (a,b) => concat_string_err(a, b)
        }
    }
}
