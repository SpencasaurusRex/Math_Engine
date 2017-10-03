use super::*;

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
                // A TermSum with 0 terms is simply 0
                if t.terms.len() == 0 {
                    return Expression::Constant(Constant::Int(0));
                }
                // A TermSum with 1 term is simply that term
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

impl From<Constant> for Expression {
    fn from(c: Constant) -> Expression {
        Expression::Constant(c)
    }
}

impl From<Variable> for Expression {
    fn from(v: Variable) -> Expression {
        Expression::Variable(v)
    }
}

impl From<TermSum> for Expression {
    fn from(t: TermSum) -> Expression {
        Expression::TermSum(t)
    }
}

impl From<Term> for Expression {
    fn from(t: Term) -> Expression {
        Expression::Term(t)
    }
}

impl From<BasicTerm> for Expression {
    fn from(b: BasicTerm) -> Expression {
        Expression::BasicTerm(b)
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