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

impl State {
    pub fn new() -> State {
        State {
            action: Actions::NONE,
            xs: String::from(""),
            l: 0.0, x: 0.0, y: 0.0, z: 0.0, t: 0.0,
            regten: 0,
            regs: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            decimals: 2
        }
    }

    pub fn stackup(&mut self) {
        self.t = self.z;
        self.z = self.y;
        self.y = self.x;
    }

    pub fn stackdown(&mut self) {
        self.x = self.y;
        self.y = self.z;
        self.z = self.t;
    }

    pub fn insert(&mut self, c: usize) {
        if self.action == Actions::STO {
            self.action = Actions::ROLL;
            self.regs[self.regten + c] = self.x;
            self.regten = 0;
            return self.show();
        }
        else if self.action == Actions::RCL {
            self.action = Actions::ROLL;
            self.x = self.regs[self.regten + c];
            self.regten = 0;
            return self.show();
        }
        else if self.action == Actions::ROLL {
            self.stackup();
        }
        self.regten = 0;

        if self.action == Actions::NONE || self.action == Actions::ROLL {
            self.xs = String::from("");
        }

        self.xs = format!("{}{}", self.xs, c);
        self.x = self.xs.parse().unwrap();

        self.action = Actions::INSERT;
        //self.show();
    }

    pub fn show(&mut self) {
        println!("Display: {}", if self.action == Actions::INSERT { self.xs.to_string() } else { self.x.to_string() });
    }

    pub fn decimal(&mut self) {
        if self.action == Actions::STO || self.action == Actions::RCL {
            return self.regten = 10;
        }
        else if self.action == Actions::NONE || self.action == Actions::ROLL {
            self.action = Actions::INSERT;
            self.stackup();
            self.xs = String::from("");
        }

        if !self.xs.contains(".") {
            self.xs = format!("{}.", self.xs);
        }
        self.show();
    }

    pub fn enter(&mut self) {
        self.action = Actions::NONE;
        self.stackup();
        self.show();
    }

    pub fn add(&mut self) {
        self.action = Actions::ROLL;
        self.l = self.x;
        self.stackdown();
        self.x = self.x + self.l;
        self.show();
    }

    pub fn sub(&mut self) {
        self.action = Actions::ROLL;
        self.l = self.x;
        self.stackdown();
        self.x = self.x - self.l;
        self.show();
    }

    pub fn mul(&mut self) {
        self.action = Actions::ROLL;
        self.l = self.x;
        self.stackdown();
        self.x = self.x * self.l;
        self.show();
    }

    pub fn div(&mut self) {
        self.action = Actions::ROLL;
        self.l = self.x;
        self.stackdown();
        self.x = self.x / self.l;
        self.show();
    }

    pub fn swap(&mut self) {
        self.action = Actions::ROLL;
        let current = self.x;
        self.x = self.y;
        self.y = current;
        self.show();
    }

    pub fn rotate(&mut self) {
        self.action = Actions::ROLL;
        let current = self.x;
        self.stackdown();
        self.t = current;
        self.show();
    }

    pub fn clx(&mut self) {
        self.action = Actions::NONE;
        self.x = 0.0;
        self.show();
    }

    pub fn chs(&mut self) {
        self.xs = format!("-{}", self.xs);
        self.x = -1.0 * self.x;
        self.show();
    }

    pub fn lstx(&mut self) {
        self.action = Actions::ROLL;
        self.stackup();
        self.x = self.l;
        self.show();
    }

    pub fn percent(&mut self) {
        self.action = Actions::ROLL;
        self.l = self.x;
        self.x = self.x / 100.0 * self.y;
        self.show();
    }

    pub fn deltapercent(&mut self) {
        self.action = Actions::ROLL;
        self.l = self.x;
        self.x = (self.x - self.y) * 100.0 / self.y;
        self.show();
    }

    pub fn sto(&mut self) {
        self.action = Actions::STO;
    }

    pub fn rcl(&mut self) {
        if self.action != Actions::NONE {
            self.stackup();
        }
        self.action = Actions::RCL;
    }

    pub fn square(&mut self) {
        self.action = Actions::ROLL;
        self.l = self.x;
        self.x = f32::powi(self.x, 2);
        self.show();
    }

    pub fn power(&mut self) {
        self.action = Actions::ROLL;
        self.l = self.x;
        self.stackdown();
        self.x = f32::powf(self.x, self.l);
        self.show();
    }

    pub fn root(&mut self) {
        self.action = Actions::ROLL;
        self.l = self.x;
        self.x = f32::sqrt(self.x);
        self.show();
    }

    pub fn overx(&mut self) {
        self.action = Actions::ROLL;
        self.l = self.x;
        self.x = 1.0 / self.x;
        self.show();
    }

    pub fn exp(&mut self) {
        self.action = Actions::ROLL;
        self.l = self.x;
        self.x = f32::exp(self.x);
        self.show();
    }
}
