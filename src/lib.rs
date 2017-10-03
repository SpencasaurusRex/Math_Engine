#[allow(unused_imports)]
#[macro_use] extern crate assert_approx_eq;

use std::fmt;

pub use equation::Equation as Equation;
pub use expression::Expression as Expression;
pub use constant::Constant as Constant;
pub use term_sum::TermSum as TermSum;
pub use term::Term as Term;
pub use basic_term::BasicTerm as BasicTerm;
pub use function::Function as Function;
pub use function::FunctionType as FunctionType;
pub use variable::Variable as Variable;
pub use assignment::Assignment as Assignment;

mod equation;
mod expression;
mod constant;
mod term_sum;
mod term;
mod basic_term;
mod function;
mod variable;
mod assignment;

/// Anything (Expression or smaller) that can be evaluated
pub trait Evaluate {
    fn evaluate_f64(&self, _:&Vec<Assignment>) -> Result<f64,String>;
}

/// Takes two results, and returns the string containing both errors concatenated.
fn concat_string_err<T>(a: Result<T, String>, b: Result<T, String>) -> Result<T, String> {
    match (a, b) {
        (Ok(_), Ok(_)) => { panic!("Expected at least one error") },
        (Ok(_), Err(b)) => { return Err(b); },
        (Err(a), Ok(_)) => { return Err(a); },
        (Err(a), Err(b)) => {return Err(format!("{}\n{}", a, b)); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn float_eval() {
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
            assert_approx_eq!(ans, 49.91831543f64);
        } 
        else if let Err(err) = ans {
            panic!(err);
        }
    }
}