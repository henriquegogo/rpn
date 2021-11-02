use rpn::*;

#[test]
fn test_stackdown() {
    let mut state = State {
        action: Actions::NONE,
        xs: String::from(""),
        l: 0.0, x: 1.0, y: 2.0, z: 3.0, t: 4.0,
        regten: 0,
        regs: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        decimals: 2
    };

    let initial_x = state.x;
    stackdown(&mut state);
    let final_x = state.x;

    assert_eq!(1.0, initial_x);
    assert_eq!(2.0, final_x);
}
