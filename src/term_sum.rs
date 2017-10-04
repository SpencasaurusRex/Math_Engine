use super::*;

// A collection of terms to be added
#[derive(Clone)]
pub struct TermSum {
    pub terms: Vec<Term>
}

impl TermSum {
    pub fn new(terms: Vec<Term>) -> TermSum {
        TermSum {
            terms: terms,
        }
    }
}

impl From<Term> for TermSum {
    fn from(term: Term) -> TermSum {
        TermSum::new(vec![term])
    }
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
