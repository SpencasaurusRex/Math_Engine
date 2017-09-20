/// Two expressions set equal
#[derive(Clone)]
struct Equation {
    left:  MathThing,
    right: MathThing
}

/// A collection of terms to be added
#[derive(Clone)]
struct Expression {
    terms: Vec<Term>
}

/// A collection of basic terms to be multiplied
#[derive(Clone)]
struct Term {
    basic_terms: Vec<BasicTerm>
}

/// An elemental to a power (ex: x^2)
#[derive(Clone)]
// TODO: Would it be better to have MathThing ^ MathThing?
struct BasicTerm {
    base: Elemental,
    power: MathThing
}

#[derive(Clone)]
struct Function{
    args: MathThing,
    func_type: FunctionType
}

#[derive(Clone)]
struct Variable {
    name: String
}

#[derive(Clone)]
enum Constant {
    E,
    PI,
    Decimal(f64),
    Int(i64),
    // TODO add fractional data type
}

// Either a function, or any term with no multiplication involved (ex: sin(x), x, 1)
#[derive(Clone)]
enum Elemental {
    Function,
    Variable,

}

#[derive(Clone)]
enum FunctionType{
    Pow(MathThing),
    Inverse,
    // TODO a LOT more functions to come, (trig, derivative, integral, def. integral, log, ln, abs, etc..)
}

/// TODO: Come up with a better name >_>
#[derive(Clone)]
enum MathThing {
    Expression,
    Term,
    BasicTerm,
    Variable,
    Constant,
    Function
}

#[derive(Clone)]
struct Assignment {
    var: Variable,
    constant: Constant
}

trait EvaluateF64 {
    fn evaluate_f64(&self, a: &Assignment) -> Option<f64>;
}

impl EvaluateF64 for Elemental {
    fn evaluate_f64(&self, _: &Assignment) -> Option<f64>{
        None
    }
}

impl EvaluateF64 for MathThing {
    fn evaluate_f64(&self, _: &Assignment) -> Option<f64> {
        None
    }
}

impl EvaluateF64 for Expression {
    fn evaluate_f64(&self, a: &Assignment) -> Option<f64> {
        // Evaluate all Terms and add
        let mut result = 0f64;
        for i in 0..self.terms.len() {
            if let Some(x) = self.terms[i].evaluate_f64(a) {
                result += x;
            }
            else {
                return None;
            }
        }
        Some(result)
    }
}

impl EvaluateF64 for Term {
    fn evaluate_f64(&self, a: &Assignment) -> Option<f64> {
        // If there are no terms return 0
        if self.basic_terms.len() == 0 { return Some(0f64); }
        
        // Evaluate all BasicTerms and multiply
        let mut result = 1f64;
        for i in 0..self.basic_terms.len() {
            if let Some(x) = self.basic_terms[i].evaluate_f64(a) {
                result *= x;
            }
            else {
                return None; 
            }
        }
        Some(result)
    }
}

impl EvaluateF64 for BasicTerm {
    fn evaluate_f64(&self, a: &Assignment) -> Option<f64> {
        // TODO special cases (ex. power = 0, base =/= 0, ans = 1)
        if let (Some(base), Some(power)) = (self.base.evaluate_f64(a),self.power.evaluate_f64(a)) {
            return Some(base.powf(power));
        }
        None
    }
}

impl EvaluateF64 for Variable {
    fn evaluate_f64(&self, a: &Assignment) -> Option<f64> {
        if self.name == a.var.name {
            return a.constant.evaluate_f64(a);
        }
        None
    }
}

impl EvaluateF64 for Constant {
    fn evaluate_f64(&self, _: &Assignment) -> Option<f64> {
        match *self {
            Constant::E => Some(std::f64::consts::E),
            Constant::PI => Some(std::f64::consts::PI),
            Constant::Decimal(x) => Some(x),
            Constant::Int(x) => Some(x as f64),
        }
    }
}

impl EvaluateF64 for Function {
    fn evaluate_f64(&self, a: &Assignment) -> Option<f64> {
        let f = self.args.evaluate_f64(a);
        match self.func_type.clone() { // TODO: Another way to avoid borrowing here?
            FunctionType::Pow(power) => {
                let power = power.evaluate_f64(a);
                if let (Some(base), Some(power)) = (f, power) {
                    return Some(base.powf(power));
                }
                None
            },
            FunctionType::Inverse => {
                if let Some(f) = f {
                    return Some(1f64 / f); 
                }
                None
            }
        }
    }
}

impl MathThing {
    /// Converts to smallest type possible
    /// Where Expression > Term > BasicTerm > [Variable, Constant, Function]
    fn simplify_type (&self) -> MathThing {
        // TODO implement
        self.clone()    
    }
}

fn main() {
    // f(x) = 2x^2
    let x = Variable {name: "x".to_string()};
    let two = MathThing::Constant;
    // let x = BasicTerm {base: x, power: two};
}