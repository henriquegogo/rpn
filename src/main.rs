use std::io::stdin;
use rpn::State;

fn is_string_numeric(str: String) -> bool {
    for c in str.chars() {
        if !c.is_numeric() && c.to_string().as_str() != "\n" {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut calc = State::new();

    loop {
        let mut input_string = String::new();
        stdin().read_line(&mut input_string).expect("Failed to read line");

        for c in input_string.chars() {
            if c.is_numeric() {
                calc.insert(c.to_string().parse::<usize>().unwrap());
            }
            else {
                match c.to_string().as_str() {
                    " " => calc.enter(),
                    "+" => calc.add(),
                    "-" => calc.sub(),
                    "*" => calc.mul(),
                    "/" => calc.div(),
                    "l" => calc.lstx(),
                    "." => calc.decimal(),
                    "%" => calc.percent(),
                    "D" => calc.deltapercent(),
                    "²" => calc.square(),
                    "^" => calc.power(),
                    "V" => calc.root(),
                    "\\" => calc.overx(),
                    "s" => calc.sto(),
                    "r" => calc.rcl(),
                    "x" => calc.clx(),
                    "<" => calc.rotate(),
                    ">" => calc.swap(),
                    "n" => calc.swap(),
                    _ => print!("")
                }
            }
        }

        if is_string_numeric(input_string) {
            calc.enter();
        }
    }
}
