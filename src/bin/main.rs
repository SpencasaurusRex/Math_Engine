extern crate math_engine;
use math_engine::*;

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