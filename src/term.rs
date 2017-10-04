use super::*;

/// A collection of basic terms to be multiplied
#[derive(Clone)]
pub struct Term {
    basic_terms: Vec<BasicTerm>
}

impl Term {
    pub fn new(basic_terms: Vec<BasicTerm>) -> Term {
        Term {
            basic_terms: basic_terms,
        }
    }
}

impl From<BasicTerm> for Term {
    fn from(b: BasicTerm) -> Term {
        Term::new(vec![b])
    }
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
        println!("Evaluating term:       {}", self);
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