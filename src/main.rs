#[derive(PartialEq)]
enum Actions { NONE, INSERT, ROLL, STO, RCL }

struct State {
    action: Actions,
    xs: String,
    l: f32, x: f32, y: f32, z: f32, t: f32,
    regten: usize,
    regs: [f32; 20],
    decimals: i8
}

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

fn stackup(state: &mut State) {
  state.t = state.z;
  state.z = state.y;
  state.y = state.x;
}

fn stackdown(state: &mut State) {
  state.x = state.y;
  state.y = state.z;
  state.z = state.t;
}

fn insert(state: &mut State, c: usize) {
  if state.action == Actions::STO {
    state.action = Actions::ROLL;
    state.regs[state.regten + c] = state.x;
    state.regten = 0;
    return show(state);
  }
  else if state.action == Actions::RCL {
    state.action = Actions::ROLL;
    state.x = state.regs[state.regten + c];
    state.regten = 0;
    return show(state);
  }
  else if state.action == Actions::ROLL {
    stackup(state);
  }
  state.regten = 0;

  if state.action == Actions::NONE || state.action == Actions::ROLL {
      state.xs = String::from("");
  }

  state.xs = format!("{}{}", state.xs, c);
  state.x = state.xs.parse().unwrap();

  state.action = Actions::INSERT;
  show(state);
}

fn show(state: &mut State) {
    println!("STATE");
}

fn decimal(state: &mut State) {
  if state.action == Actions::STO || state.action == Actions::RCL {
    return state.regten = 10;
  }
  else if state.action == Actions::NONE || state.action == Actions::ROLL {
    state.action = Actions::INSERT;
    stackup(state);
    state.xs = String::from("");
  }

  if !state.xs.contains(".") {
      state.xs = format!("{}.", state.xs);
  }
  show(state);
}

fn enter(state: &mut State) {
  state.action = Actions::NONE;
  stackup(state);
  show(state);
}

fn add(state: &mut State) {
  state.action = Actions::ROLL;
  state.l = state.x;
  stackdown(state);
  state.x = state.x + state.l;
  show(state);
}

fn sub(state: &mut State) {
  state.action = Actions::ROLL;
  state.l = state.x;
  stackdown(state);
  state.x = state.x - state.l;
  show(state);
}

fn mul(state: &mut State) {
  state.action = Actions::ROLL;
  state.l = state.x;
  stackdown(state);
  state.x = state.x * state.l;
  show(state);
}

fn div(state: &mut State) {
  state.action = Actions::ROLL;
  state.l = state.x;
  stackdown(state);
  state.x = state.x / state.l;
  show(state);
}

fn swap(state: &mut State) {
  state.action = Actions::ROLL;
  let current = state.x;
  state.x = state.y;
  state.y = current;
  show(state);
}

fn rotate(state: &mut State) {
  state.action = Actions::ROLL;
  let current = state.x;
  stackdown(state);
  state.t = current;
  show(state);
}

fn clx(state: &mut State) {
  state.action = Actions::NONE;
  state.x = 0.0;
  show(state);
}

fn chs(state: &mut State) {
  state.xs = format!("-{}", state.xs);
  state.x = -1.0 * state.x;
  show(state);
}

fn lstx(state: &mut State) {
  state.action = Actions::ROLL;
  stackup(state);
  state.x = state.l;
  show(state);
}

fn percent(state: &mut State) {
  state.action = Actions::ROLL;
  state.l = state.x;
  state.x = state.x / 100.0 * state.y;
  show(state);
}

fn deltapercent(state: &mut State) {
  state.action = Actions::ROLL;
  state.l = state.x;
  state.x = (state.x - state.y) * 100.0 / state.y;
  show(state);
}

fn sto(state: &mut State) {
  state.action = Actions::STO;
}

fn rcl(state: &mut State) {
  if state.action != Actions::NONE {
      stackup(state);
  }
  state.action = Actions::RCL;
}

fn square(state: &mut State) {
  state.action = Actions::ROLL;
  state.l = state.x;
  state.x = f32::powi(state.x, 2);
  show(state);
}

fn power(state: &mut State) {
  state.action = Actions::ROLL;
  state.l = state.x;
  stackdown(state);
  state.x = f32::powf(state.x, state.l);
  show(state);
}

fn root(state: &mut State) {
  state.action = Actions::ROLL;
  state.l = state.x;
  state.x = f32::sqrt(state.x);
  show(state);
}

fn overx(state: &mut State) {
  state.action = Actions::ROLL;
  state.l = state.x;
  state.x = 1.0 / state.x;
  show(state);
}

fn exp(state: &mut State) {
  state.action = Actions::ROLL;
  state.l = state.x;
  state.x = f32::exp(state.x);
  show(state);
}

