extern crate math_engine;
use math_engine::*;

fn main() {

    let x_var = Variable {name: "x".to_string()};
    let pow = Constant::E;
    let x_to_the_e = BasicTerm::new(x_var.clone(), pow);
    let two = Expression::Constant(Constant::Int(2));
    let two = BasicTerm::from(two);
    // TODO: Continue replacing with constructors
    let two_x_to_the_e = Term::new(vec![two, x_to_the_e]);
    let five = Expression::Constant(Constant::Int(5));
    let five = BasicTerm::from(five);
    let five = Term::from(five);
    let two_x_to_the_e_plus_five = TermSum::new(vec![two_x_to_the_e, five]);
    
    // Eval at x = pi
    let pi = Constant::Pi;
    let assignment = Assignment {var: x_var, constant: pi };
    let ans = two_x_to_the_e_plus_five.evaluate_f64(&vec![assignment]);
    if let Ok(ans) = ans {
        println!("2pi^e + 5 = {}", ans);
    } 
    else if let Err(err) = ans {
        println!("{}", err);
    }
}