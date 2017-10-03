use std::fmt;

mod equation;
mod expression;
mod constant;
mod term_sum;
mod term;
mod basic_term;
mod function;
mod variable;
mod assignment;

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

/// Anything (Expression or smaller) that can be evaluated
pub trait Evaluate {
    fn evaluate_f64(&self, _:&Vec<Assignment>) -> Result<f64,String>;
}

fn concat_string_err<T>(a: Result<T, String>, b: Result<T, String>) -> Result<T, String>{
    match (a, b) {
        (Ok(_), Ok(_)) => { panic!("Expected at least one error") },
        (Ok(_), Err(b)) => { return Err(b); },
        (Err(a), Ok(_)) => { return Err(a); },
        (Err(a), Err(b)) => {return Err(format!("{}\n{}", a, b)); }
    }
}