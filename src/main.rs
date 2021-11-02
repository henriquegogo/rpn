use rpn::*;

fn main() {
    let mut state = State {
        action: Actions::NONE,
        xs: String::from(""),
        l: 0.0, x: 0.0, y: 0.0, z: 0.0, t: 0.0,
        regten: 0,
        regs: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        decimals: 2
    };

    println!("{}", state.x);

    stackdown(&mut state);

    println!("{}", state.x);
}
