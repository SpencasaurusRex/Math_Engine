#[derive(Clone)]
enum Constant {
    E,
    Pi,
    Decimal(f64),
    Int(i32),
}

#[derive(Clone)]
enum Expression {
    Const(Constant),
    Var(Variable),
    // TODO add fractional
}

/// Two TermSums set equal
#[derive(Clone)]
struct Equation {
    left:  Expression,
    right: Expression,
}

// A collection of terms to be added
#[derive(Clone)]
struct TermSum {
    terms: Vec<Term>
}

/// A collection of basic terms to be multiplied
#[derive(Clone)]
struct Term {
    basic_terms: Vec<BasicTerm>
}

/// An elemental to a power (ex: x^2)
// TODO: I would prefer (Function/Variable/Constant) ^ power, 
// but that causes nested enums, figure out a way around this
#[derive(Clone)]
struct BasicTerm {
    base: Expression,
    power: Expression
}

#[derive(Clone)]
struct Function{
    args: Expression,
    func_type: FunctionType
}

#[derive(Clone)]
struct Variable {
    name: String
}

#[derive(Clone)]
enum FunctionType{
    Pow(Expression),
    Inverse,
    // TODO a LOT more functions to come, (trig, derivative, integral, def. integral, log, ln, abs, etc..)
}

#[derive(Clone)]
struct Assignment {
    var: Variable,
    constant: Constant
}

impl Constant {
    fn as_expression(&self) -> Expression {
        Expression::Const(self.clone())
    }
}

impl Variable {
    fn as_expression(&self) -> Expression {
        Expression::Var(self.clone())
    }
}

impl Constant {
    fn to_expression(self) -> Expression {
        Expression::Const(self)
    }
}

impl Variable {
    fn to_expression(self) -> Expression {
        Expression::Var(self)
    }
}

trait Expressable {
    fn evaluate_f64(&self, _:&Assignment) -> Option<f64>;
}

impl Expressable for Expression {
    fn evaluate_f64(&self, _:&Assignment) -> Option<f64> {
        None
    }
}

impl Expressable for TermSum {
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

impl Expressable for Term {
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

impl Expressable for BasicTerm {
    fn evaluate_f64(&self, a: &Assignment) -> Option<f64> {
        // TODO special cases (ex. power = 0, base =/= 0, ans = 1)
        if let (Some(base), Some(power)) = (self.base.evaluate_f64(a),self.power.evaluate_f64(a)) {
            return Some(base.powf(power));
        }
        None
    }
}

impl Expressable for Variable {
    fn evaluate_f64(&self, a: &Assignment) -> Option<f64> {
        if self.name == a.var.name {
            return a.constant.evaluate_f64(a);
        }
        None
    }
}

impl Expressable for Constant {
    fn evaluate_f64(&self, _: &Assignment) -> Option<f64> {
        match *self {
            Constant::E => Some(std::f64::consts::E),
            Constant::Pi => Some(std::f64::consts::PI),
            Constant::Decimal(x) => Some(x),
            Constant::Int(x) => Some(x as f64),
        }
    }
}

impl Expressable for Function {
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

impl Expression {
    /// Converts to smallest type possible
    /// Where TermSum > Term > BasicTerm > [Variable, Constant, Function]
    fn simplify_type (&self) -> Expression {
        // TODO implement
        self.clone()    
    }
}

fn main() {
    // f(x) = 2x^2
    let x_var = Variable {name: "x".to_string()};
    let two = Constant::Int(2).to_expression();
    let f = BasicTerm{base: x_var.as_expression(), power: two};
    let three = Constant::Int(3);
    let assignment = Assignment {var: x_var, constant: three };
    let ans = f.evaluate_f64(&assignment);
    if let Some(ans) = ans {
        println!("2(3)^2 = {}", ans);
    } 
    else {
        println!("Couldn't be calculated");
    }
}