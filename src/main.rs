// Two expressions set equal
#[derive(Clone)]
struct Equation {
    left:  Expression,
    right: Expression
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
struct BasicTerm {
    base: Elemental,
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
enum Constant {
    E,
    PI,
    Decimal(f64),
    Int(i64),
    // TODO add fractional data type
}

// Any term with no other multiplications than a power (ex: x^2, sin(x), 1)
#[derive(Clone)]
enum Elemental {
    Function,
    Variable,
    Constant
}

#[derive(Clone)]
enum FunctionType{
    Pow(Expression)
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

impl MathThing {
    // 
    fn simplify_type (&self) -> MathThing {
        // TODO implement
        self.clone()    
    }
}

fn main() {

}