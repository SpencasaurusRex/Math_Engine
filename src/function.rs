use super::*;

#[derive(Clone)]
pub enum FunctionType{
    Pow(Expression),
    Inverse,
    // TODO a LOT more functions to come, (trig, derivative, integral, def. integral, log, ln, abs, etc..)
}

#[derive(Clone)]
pub struct Function{
    pub args: Expression,
    pub func_type: FunctionType
}

impl Evaluate for Function {
    fn evaluate_f64(&self, a: &Vec<Assignment>) -> Result<f64,String> {
        // println!("Evaluating {}!");
        let f = self.args.evaluate_f64(a);
        match self.func_type.clone() { // TODO: Another way to avoid borrowing here?
            FunctionType::Pow(power) => {
                let power = power.evaluate_f64(a);
                match (f, power) {
                    (Ok(base), Ok(power)) => Ok(base.powf(power)),
                    (a,b) => concat_string_err(a, b)
                }
            },
            FunctionType::Inverse => {
                if let Ok(f) = f {
                    return Ok(1f64 / f); 
                }
                f
            }
        }
    }
}
