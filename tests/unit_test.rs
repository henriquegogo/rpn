use rpn::State;

#[test]
fn test_stackup() {
    let mut calc = State::new();
    calc.x = 1.0; calc.y = 2.0; calc.z = 3.0; calc.t = 4.0;
    calc.stackup();

    assert_eq!(0.0, calc.l);
    assert_eq!(1.0, calc.x);
    assert_eq!(1.0, calc.y);
    assert_eq!(2.0, calc.z);
    assert_eq!(3.0, calc.t);
}

#[test]
fn test_stackdown() {
    let mut calc = State::new();
    calc.x = 1.0; calc.y = 2.0; calc.z = 3.0; calc.t = 4.0;
    calc.stackdown();

    assert_eq!(0.0, calc.l);
    assert_eq!(2.0, calc.x);
    assert_eq!(3.0, calc.y);
    assert_eq!(4.0, calc.z);
    assert_eq!(4.0, calc.t);
}

#[test]
fn test_add() {
    let mut calc = State::new();
    calc.x = 2.0; calc.y = 3.0;
    calc.add();

    assert_eq!(0.0, calc.y);
    assert_eq!(5.0, calc.x);
    assert_eq!(2.0, calc.l);
}

#[test]
fn test_sub() {
    let mut calc = State::new();
    calc.x = 2.0; calc.y = 3.0;
    calc.sub();

    assert_eq!(0.0, calc.y);
    assert_eq!(1.0, calc.x);
    assert_eq!(2.0, calc.l);
}

#[test]
fn test_mul() {
    let mut calc = State::new();
    calc.x = 2.0; calc.y = 3.0;
    calc.mul();

    assert_eq!(0.0, calc.y);
    assert_eq!(6.0, calc.x);
    assert_eq!(2.0, calc.l);
}

#[test]
fn test_div() {
    let mut calc = State::new();
    calc.x = 2.0; calc.y = 3.0;
    calc.div();

    assert_eq!(0.0, calc.y);
    assert_eq!(1.5, calc.x);
    assert_eq!(2.0, calc.l);
}

#[test]
fn test_swap() {
    let mut calc = State::new();
    calc.x = 1.0; calc.y = 2.0; calc.z = 3.0; calc.t = 4.0;
    calc.swap();

    assert_eq!(0.0, calc.l);
    assert_eq!(2.0, calc.x);
    assert_eq!(1.0, calc.y);
    assert_eq!(3.0, calc.z);
    assert_eq!(4.0, calc.t);
}

#[test]
fn test_rotate() {
    let mut calc = State::new();
    calc.x = 1.0; calc.y = 2.0; calc.z = 3.0; calc.t = 4.0;
    calc.rotate();

    assert_eq!(0.0, calc.l);
    assert_eq!(2.0, calc.x);
    assert_eq!(3.0, calc.y);
    assert_eq!(4.0, calc.z);
    assert_eq!(1.0, calc.t);
}

#[test]
fn test_clx() {
    let mut calc = State::new();
    calc.x = 1.0; calc.y = 2.0; calc.z = 3.0; calc.t = 4.0;
    calc.clx();

    assert_eq!(0.0, calc.l);
    assert_eq!(0.0, calc.x);
    assert_eq!(2.0, calc.y);
    assert_eq!(3.0, calc.z);
    assert_eq!(4.0, calc.t);
}

#[test]
fn test_chs() {
    let mut calc = State::new();
    calc.x = 1.0;
    calc.chs();

    assert_eq!(-1.0, calc.x);
}

#[test]
fn test_lstx() {
    let mut calc = State::new();
    calc.x = 2.0; calc.y = 3.0;

    assert_eq!(0.0, calc.l);
    calc.add();
    assert_eq!(2.0, calc.l);
}

#[test]
fn test_percent() {
    let mut calc = State::new();
    calc.x = 2.0; calc.y = 3.0;
    calc.percent();

    assert_eq!(3.0, calc.y);
    assert_eq!(0.06, calc.x);
    assert_eq!(2.0, calc.l);
}

#[test]
fn test_deltapercent() {
    let mut calc = State::new();
    calc.x = 3.0; calc.y = 2.0;
    calc.deltapercent();

    assert_eq!(2.0, calc.y);
    assert_eq!(50.0, calc.x);
    assert_eq!(3.0, calc.l);
}

#[test]
fn test_square() {
    let mut calc = State::new();
    calc.x = 2.0; calc.y = 3.0;
    calc.square();

    assert_eq!(3.0, calc.y);
    assert_eq!(4.0, calc.x);
    assert_eq!(2.0, calc.l);
}

#[test]
fn test_power() {
    let mut calc = State::new();
    calc.x = 2.0; calc.y = 3.0;
    calc.power();

    assert_eq!(0.0, calc.y);
    assert_eq!(9.0, calc.x);
    assert_eq!(2.0, calc.l);
}

#[test]
fn test_root() {
    let mut calc = State::new();
    calc.x = 9.0;
    calc.root();

    assert_eq!(0.0, calc.y);
    assert_eq!(3.0, calc.x);
    assert_eq!(9.0, calc.l);
}

#[test]
fn test_overx() {
    let mut calc = State::new();
    calc.x = 2.0;
    calc.overx();

    assert_eq!(0.0, calc.y);
    assert_eq!(0.5, calc.x);
    assert_eq!(2.0, calc.l);
}
