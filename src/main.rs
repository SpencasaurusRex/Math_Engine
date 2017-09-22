/// Anything (Expression or smaller) that can be evaluated
pub trait Evaluate {
    // TODO change to be vector of assignments
    fn evaluate_f64(&self, _:&Assignment) -> Result<f64,String>;
}

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
    fn evaluate_f64(&self, _: &Assignment) -> Result<f64,String> {
        println!("Evaluating constant!");
        match *self {
            Constant::E => Ok(std::f64::consts::E),
            Constant::Pi => Ok(std::f64::consts::PI),
            Constant::Decimal(x) => Ok(x),
            Constant::Int(x) => Ok(x as f64),
        }
    }
}

// Any mathematical expression
#[derive(Clone)]
pub enum Expression {
    Constant(Constant),
    Variable(Variable),
    BasicTerm(BasicTerm),
    TermSum(TermSum),
}

/// Two TermSums set equal
#[derive(Clone)]
pub struct Equation {
    left:  Expression,
    right: Expression,
}

// A collection of terms to be added
#[derive(Clone)]
pub struct TermSum {
    terms: Vec<Term>
}

/// A collection of basic terms to be multiplied
#[derive(Clone)]
pub struct Term {
    basic_terms: Vec<BasicTerm>
}

/// An elemental to a power (ex: x^2)
#[derive(Clone)]
pub struct BasicTerm {
    base: Box<Expression>,
    power: Box<Expression>
}

#[derive(Clone)]
pub struct Function{
    args: Expression,
    func_type: FunctionType
}

#[derive(Clone)]
pub struct Variable {
    name: String
}

#[derive(Clone)]
pub enum FunctionType{
    Pow(Expression),
    Inverse,
    // TODO a LOT more functions to come, (trig, derivative, integral, def. integral, log, ln, abs, etc..)
}

#[derive(Clone)]
pub struct Assignment {
    var: Variable,
    constant: Constant
}

impl Variable {
    pub fn as_expression(&self) -> Expression {
        Expression::Variable(self.clone())
    }

    pub fn to_expression(self) -> Expression {
        Expression::Variable(self)
    }
}

impl Evaluate for Expression {
    fn evaluate_f64(&self, a:&Assignment) -> Result<f64,String> {
        match self.clone() { 
            Expression::Variable(v) => {return v.evaluate_f64(a);},
            Expression::Constant(c) => {return c.evaluate_f64(a);},
            Expression::BasicTerm(b) => {return b.evaluate_f64(a);},
            Expression::TermSum(t) => {return t.evaluate_f64(a);},
        }
    }
}

impl Evaluate for TermSum {
    fn evaluate_f64(&self, a: &Assignment) -> Result<f64,String> {
        println!("Evaluating TermSum!");
        // Evaluate all Terms and add
        let mut result = 0f64;
        for i in 0..self.terms.len() {
            let eval = self.terms[i].evaluate_f64(a);
            if let Ok(x) = eval {
                result += x;
            }
            else {
                return eval;
            }
        }
        Ok(result)
    }
}

impl Evaluate for Term {
    fn evaluate_f64(&self, a: &Assignment) -> Result<f64,String> {
        println!("Evaluating Term!");
        // If there are no terms return 0
        if self.basic_terms.len() == 0 { return Ok(0f64); }
        
        // Evaluate all BasicTerms and multiply
        let mut result = 1f64;
        for i in 0..self.basic_terms.len() {
            let eval = self.basic_terms[i].evaluate_f64(a);
            if let Ok(x) = eval {
                result *= x;
            }
            else {
                return eval; 
            }
        }
        Ok(result)
    }
}

impl Evaluate for BasicTerm {
    fn evaluate_f64(&self, a: &Assignment) -> Result<f64,String> {
        println!("Evaluating Basic Term!");
        // TODO special cases (ex. power = 0, base =/= 0, ans = 1)
        let base_eval = self.base.evaluate_f64(a);
        let power_eval = self.power.evaluate_f64(a);
        match (base_eval, power_eval) {
            (Ok(base), Ok(power)) => Ok(base.powf(power)),
            (a,b) => concat_string_err(a, b)
        }
    }
}

impl Evaluate for Variable {
    fn evaluate_f64(&self, a: &Assignment) -> Result<f64,String> {
        println!("Evaluating variable!");
        if self.name == a.var.name {
            return a.constant.evaluate_f64(a);
        }
        // TODO provide more info
        Err ("Incorrect variable supplied".to_string())
    }
}

impl Evaluate for Function {
    fn evaluate_f64(&self, a: &Assignment) -> Result<f64,String> {
        println!("Evaluating function!");
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

impl Expression {
    /// Converts to smallest type possible
    /// Where TermSum > Term > BasicTerm > [Variable, Constant, Function]
    pub fn simplify_type (&self) -> Expression {
        // TODO implement
        self.clone()    
    }
}

fn concat_string_err<T>(a: Result<T, String>, b: Result<T, String>) -> Result<T, String>{
    let mut err : String;
    match (a, b) {
        (Ok(_), Ok(_)) => { panic!("Expected at least one error") },
        (Ok(_), Err(b)) => { return Err(b); },
        (Err(a), Ok(_)) => { return Err(a); },
        (Err(a), Err(b)) => {return Err(format!("{}\n{}", a, b)); }
    }
}

fn main() {

    // f(x) = 2x^2
    let x_var = Variable {name: "x".to_string()};
    let pow = Constant::E;
    let expression = BasicTerm{
        base: Box::new(x_var.as_expression()), 
        power: Box::new(pow.as_expression())
    };
    
    // Eval at x = pi
    let pi = Constant::Pi;
    let assignment = Assignment {var: x_var, constant: pi };
    let ans = expression.evaluate_f64(&assignment);
    if let Ok(ans) = ans {
        println!("pi^e = {}", ans);
    } 
    else if let Err(err) = ans {
        println!("{}", err);
    }
}