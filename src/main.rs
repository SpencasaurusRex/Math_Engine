use std::fmt;

/// Anything (Expression or smaller) that can be evaluated
pub trait Evaluate {
    fn evaluate_f64(&self, _:&Vec<Assignment>) -> Result<f64,String>;
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

// Any mathematical expression
#[derive(Clone)]
pub enum Expression {
    Constant(Constant),
    Variable(Variable),
    TermSum(TermSum),
    Term(Term),
    BasicTerm(BasicTerm),
}

impl Expression {
    /// Converts to smallest type possible
    /// Where TermSum > Term > BasicTerm > [Variable, Constant, Function]
    pub fn simplify_type (&self) -> Expression {
        // TODO implement
        match *self {
            Expression::TermSum(ref t) => {
                if t.terms.len() == 0 {
                    return Expression::Constant(Constant::Int(0));
                }
                else if t.terms.len() == 1 {
                    let term = t.terms.first();
                    if let Some(term) = term {
                        // TODO: Avoid clone here
                        return Expression::Term(term.clone()).simplify_type(); 
                    }
                }
            }
            _ => {}
        }

        self.clone()    
    }
}

impl Evaluate for Expression {
    fn evaluate_f64(&self, a:&Vec<Assignment>) -> Result<f64,String> {
        match self.clone() { 
            Expression::Variable(v) => {return v.evaluate_f64(a);},
            Expression::Constant(c) => {return c.evaluate_f64(a);},
            Expression::BasicTerm(b) => {return b.evaluate_f64(a);},
            Expression::TermSum(t) => {return t.evaluate_f64(a);},
            Expression::Term(t) => {return t.evaluate_f64(a);},
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::Constant(ref c) => c.fmt(f),
            Expression::Variable(ref v) => v.fmt(f),
            Expression::BasicTerm(ref b) => b.fmt(f),
            Expression::TermSum(ref t) => t.fmt(f),
            Expression::Term(ref t) => t.fmt(f),
        }
    }
}

/// Two Expressions set equal
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

impl fmt::Display for TermSum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = "".to_string();
        for (i, term) in self.terms.iter().enumerate() {
            if i == 0 {
                result = format!("{}", term);
            }
            else {
                result = format!("{} + {}", term, result)
            }
        }
        write!(f, "{}", result)
    }
}

impl Evaluate for TermSum {
    fn evaluate_f64(&self, a: &Vec<Assignment>) -> Result<f64,String> {
        println!("Evaluating {}", self);
        // Evaluate all Terms and add
        let mut result = 0f64;
        for term in self.terms.iter() {
            let eval = term.evaluate_f64(a);
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

/// A collection of basic terms to be multiplied
#[derive(Clone)]
pub struct Term {
    basic_terms: Vec<BasicTerm>
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = "".to_string();
        for (i, basic_term) in self.basic_terms.iter().enumerate() {
            if i == 0 {
                result = format!("{}", basic_term);
            }
            else {
                result = format!("{}*{}", result, basic_term);
            }
        }
        write!(f, "{}", result)
    }
}

impl Evaluate for Term {
    fn evaluate_f64(&self, a: &Vec<Assignment>) -> Result<f64,String> {
        println!("Evaluating {}", self);
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

/// An expression to a power (ex: x^2)
#[derive(Clone)]
pub struct BasicTerm {
    base: Box<Expression>,
    power: Box<Expression>
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
        println!("Evaluating {}", self);
        // TODO special cases (ex. power = 0, base =/= 0, ans = 1)
        let base_eval = self.base.evaluate_f64(a);
        let power_eval = self.power.evaluate_f64(a);
        match (base_eval, power_eval) {
            (Ok(base), Ok(power)) => Ok(base.powf(power)),
            (a,b) => concat_string_err(a, b)
        }
    }
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

impl Variable {
    pub fn as_expression(&self) -> Expression {
        Expression::Variable(self.clone())
    }

    pub fn to_expression(self) -> Expression {
        Expression::Variable(self)
    }
}

impl Evaluate for Variable {
    fn evaluate_f64(&self, assignments: &Vec<Assignment>) -> Result<f64,String> {
        println!("Evaluating {}", self);
        for assignment in assignments.iter() {
            if self.name == assignment.var.name {
                return assignment.constant.evaluate_f64(assignments);
            }
        }
        // TODO provide more info
        Err ("Incorrect variable supplied".to_string())
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
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

fn concat_string_err<T>(a: Result<T, String>, b: Result<T, String>) -> Result<T, String>{
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
    let two = Constant::Int(2);
    let one = Constant::Int(1);
    let five = Constant::Int(5);

    let two = BasicTerm {
        base: Box::new(two.as_expression()),
        power: Box::new(one.as_expression()),
    };
    let five = BasicTerm {
        base: Box::new(five.as_expression()),
        power: Box::new(one.as_expression()),
    };
    let x_squared = BasicTerm {
        base: Box::new(x_var.as_expression()), 
        power: Box::new(pow.as_expression())
    };
    let two_x_squared = Term {
        basic_terms: vec![two, x_squared],
    };
    let five = Term {
        basic_terms: vec![five],
    };
    let two_x_squared_plus_five = TermSum {
        terms: vec![two_x_squared, five]
    };
    
    // Eval at x = pi
    let pi = Constant::Pi;
    let assignment = Assignment {var: x_var, constant: pi };
    let ans = two_x_squared_plus_five.evaluate_f64(&vec![assignment]);
    if let Ok(ans) = ans {
        println!("2(pi^e) + 5 = {}", ans);
    } 
    else if let Err(err) = ans {
        println!("{}", err);
    }
}