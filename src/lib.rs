#[derive(PartialEq)]
pub enum Actions { NONE, INSERT, ROLL, STO, RCL }

pub struct State {
    pub action: Actions,
    pub xs: String,
    pub l: f32, pub x: f32, pub y: f32, pub z: f32, pub t: f32,
    pub regten: usize,
    pub regs: [f32; 20],
    pub decimals: i8
}

pub fn stackup(state: &mut State) {
  state.t = state.z;
  state.z = state.y;
  state.y = state.x;
}

pub fn stackdown(state: &mut State) {
  state.x = state.y;
  state.y = state.z;
  state.z = state.t;
}

pub fn insert(state: &mut State, c: usize) {
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

pub fn show(state: &mut State) {
    println!("Display: {} {}", state.xs, state.decimals);
}

pub fn decimal(state: &mut State) {
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

pub fn enter(state: &mut State) {
  state.action = Actions::NONE;
  stackup(state);
  show(state);
}

pub fn add(state: &mut State) {
  state.action = Actions::ROLL;
  state.l = state.x;
  stackdown(state);
  state.x = state.x + state.l;
  show(state);
}

pub fn sub(state: &mut State) {
  state.action = Actions::ROLL;
  state.l = state.x;
  stackdown(state);
  state.x = state.x - state.l;
  show(state);
}

pub fn mul(state: &mut State) {
  state.action = Actions::ROLL;
  state.l = state.x;
  stackdown(state);
  state.x = state.x * state.l;
  show(state);
}

pub fn div(state: &mut State) {
  state.action = Actions::ROLL;
  state.l = state.x;
  stackdown(state);
  state.x = state.x / state.l;
  show(state);
}

pub fn swap(state: &mut State) {
  state.action = Actions::ROLL;
  let current = state.x;
  state.x = state.y;
  state.y = current;
  show(state);
}

pub fn rotate(state: &mut State) {
  state.action = Actions::ROLL;
  let current = state.x;
  stackdown(state);
  state.t = current;
  show(state);
}

pub fn clx(state: &mut State) {
  state.action = Actions::NONE;
  state.x = 0.0;
  show(state);
}

pub fn chs(state: &mut State) {
  state.xs = format!("-{}", state.xs);
  state.x = -1.0 * state.x;
  show(state);
}

pub fn lstx(state: &mut State) {
  state.action = Actions::ROLL;
  stackup(state);
  state.x = state.l;
  show(state);
}

pub fn percent(state: &mut State) {
  state.action = Actions::ROLL;
  state.l = state.x;
  state.x = state.x / 100.0 * state.y;
  show(state);
}

pub fn deltapercent(state: &mut State) {
  state.action = Actions::ROLL;
  state.l = state.x;
  state.x = (state.x - state.y) * 100.0 / state.y;
  show(state);
}

pub fn sto(state: &mut State) {
  state.action = Actions::STO;
}

pub fn rcl(state: &mut State) {
  if state.action != Actions::NONE {
      stackup(state);
  }
  state.action = Actions::RCL;
}

pub fn square(state: &mut State) {
  state.action = Actions::ROLL;
  state.l = state.x;
  state.x = f32::powi(state.x, 2);
  show(state);
}

pub fn power(state: &mut State) {
  state.action = Actions::ROLL;
  state.l = state.x;
  stackdown(state);
  state.x = f32::powf(state.x, state.l);
  show(state);
}

pub fn root(state: &mut State) {
  state.action = Actions::ROLL;
  state.l = state.x;
  state.x = f32::sqrt(state.x);
  show(state);
}

pub fn overx(state: &mut State) {
  state.action = Actions::ROLL;
  state.l = state.x;
  state.x = 1.0 / state.x;
  show(state);
}

pub fn exp(state: &mut State) {
  state.action = Actions::ROLL;
  state.l = state.x;
  state.x = f32::exp(state.x);
  show(state);
}
