use rpn::State;

#[test]
fn test_stackdown() {
    let mut calc = State::new();
    calc.x = 1.0; calc.y = 2.0; calc.z = 3.0; calc.t = 4.0;

    let initial_x = calc.x;
    calc.stackdown();
    let final_x = calc.x;

    assert_eq!(1.0, initial_x);
    assert_eq!(2.0, final_x);
}
