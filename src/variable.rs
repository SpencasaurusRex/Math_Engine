use super::*;

#[derive(Clone)]
pub struct Variable {
    pub name: String
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
        println!("Evaluating variable    {}", self);
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
