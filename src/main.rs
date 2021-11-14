use rpn::State;

fn main() {
    let mut calc = State::new();

    println!("{}", calc.x);

    calc.stackdown();

    println!("{}", calc.x);
}
