use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt").trim()
        .lines()
        .map(|l| l.parse::<Op>().unwrap())
        .collect::<Vec<_>>();


    part_one(&input);
}

#[derive(Debug, Clone, PartialEq)]
enum RunState {
    Continue(usize),
    OutOfBounds(isize),
    Halt,
}

impl RunState {
    fn set_to_next(&mut self) {
        let cur = match self {
            &mut RunState::Continue(ref idx) => *idx,
            other => panic!("set_to_next called on halted program {:?}", other),
        };
        let next = cur + 1;
        *self = RunState::Continue(next);
    }
}

struct ProgramState {
    ops: Vec<Op>,
    registers: [isize; 16],
    last_sound: Option<isize>,
    run_state: RunState,
}

impl ProgramState {
    fn new(ops: &[Op]) -> Self {
        ProgramState {
            ops: ops.to_owned(),
            registers: [0isize; 16],
            last_sound: None,
            run_state: RunState::Continue(0),
        }
    }

    fn run(&mut self) -> Result<isize, String> {
        loop {
            let next_op = match self.run_state {
                RunState::Continue(idx) if idx <= self.ops.len() => idx,
                RunState::Halt => return Ok(self.last_sound.unwrap()),
                RunState::Continue(idx) => return Err(format!("out of bounds: {}", idx)),
                RunState::OutOfBounds(idx) => return Err(format!("out of bounds: {}", idx)),
            };

            let op = self.ops[next_op];
            self.execute(&op);
        }
    }

    fn execute(&mut self, op: &Op) {
        match op {
            &Op::Sound(ref val) => {
                let sound_val = self.get_value(val);
                self.last_sound = Some(sound_val);
                self.run_state.set_to_next();
            }
            &Op::Set(ref reg, ref val) => {
                let reg = reg.get_register().unwrap();
                let val = self.get_value(val);
                self.set_reg(reg, val);
                self.run_state.set_to_next();
            }
            &Op::Add(ref reg, ref val) => {
                let cur_val = self.get_value(reg);
                let reg = reg.get_register().unwrap();
                let val = self.get_value(val);
                self.set_reg(reg, cur_val + val);
                self.run_state.set_to_next();
            }
            &Op::Mul(ref reg, ref val) => {
                let cur_val = self.get_value(reg);
                let reg = reg.get_register().unwrap();
                let val = self.get_value(val);
                self.set_reg(reg, cur_val * val);
                self.run_state.set_to_next();
            }
            &Op::Mod(ref reg, ref val) => {
                let cur_val = self.get_value(reg);
                let reg = reg.get_register().unwrap();
                let val = self.get_value(val);
                self.set_reg(reg, cur_val % val);
                self.run_state.set_to_next();
            }
            &Op::Recover(ref val) => {
                let cur_val = self.get_value(val);
                if cur_val != 0 {
                    self.run_state = RunState::Halt;
                } else {
                    self.run_state.set_to_next();
                }
            }
            &Op::Jump(ref reg, ref val) => {
                let reg_val = self.get_value(reg);
                if reg_val > 0 {
                    let val = self.get_value(val);
                    self.jump(val);
                } else {
                    self.run_state.set_to_next();
                }
            }
            _ => panic!(),

        }

    }

    fn set_reg(&mut self, reg: Register, value: isize) {
        let idx = reg.to_digit(36).unwrap() - 10;
        self.registers[idx as usize] = value;
    }


    fn get_value(&self, value: &Value) -> isize {
        match *value {
            Value::Literal(int) => int,
            Value::Register(reg) => {
                let idx = reg.to_digit(36).unwrap() - 10;
                self.registers[idx as usize]
            }
        }
    }

    fn jump(&mut self, offset: isize) {
        let cur = match &self.run_state {
            &RunState::Continue(ref idx) => *idx,
            other => panic!("set_to_next called on halted program {:?}", other),
        };
        let next = cur as isize + offset;
        if next < 0 {
            self.run_state = RunState::OutOfBounds(next);
        } else {
            self.run_state = RunState::Continue(next as usize);
        }
    }
}

fn part_one(ops: &[Op]) {
    let mut state = ProgramState::new(ops);
    let result = state.run();
    println!("part one: {:?}", result);
    //let mut next_instruct = 0usize;

    // we need to track when sounds are played;
    // of our next position;
    // and of the termination conditions
}

type Register = char;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Value {
    Register(Register),
    Literal(isize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Sound(Value),
    Set(Value, Value),
    Add(Value, Value),
    Mul(Value, Value),
    Mod(Value, Value),
    Recover(Value),
    Jump(Value, Value),
}

impl Value {
    fn get_register(&self) -> Option<Register> {
        match *self {
            Value::Register(r) => Some(r),
            _ => None,
        }
    }
}

impl FromStr for Value {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(int) = s.parse::<isize>() {
            Ok(Value::Literal(int))
        } else {
            let reg = s.parse::<char>().map_err(|e| format!("{}", e))?;
            Ok(Value::Register(reg))
        }
    }
}

impl FromStr for Op {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s_iter = s.split_whitespace();
        let op_name = s_iter.next().unwrap();
        let reg = s_iter.next().unwrap().parse::<Value>().unwrap();
        let val = s_iter.next().map(|n| n.parse::<Value>().unwrap());
        match op_name {
            "snd" => Ok(Op::Sound(reg)),
            "set" => Ok(Op::Set(reg, val.unwrap())),
            "add" => Ok(Op::Add(reg, val.unwrap())),
            "mul" => Ok(Op::Mul(reg, val.unwrap())),
            "mod" => Ok(Op::Mod(reg, val.unwrap())),
            "rcv" => Ok(Op::Recover(reg)),
            "jgz" => Ok(Op::Jump(reg, val.unwrap())),
            other => panic!("illegal instruction name '{}'", other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: &str = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";

    #[test]
    fn parse_test() {
        let one_inp = "jgz a -1";
        let op = one_inp.parse::<Op>();
        assert_eq!(op, Ok(Op::Jump(Value::Register('a'), Value::Literal(-1))));

        let one_inp = "rcv a";
        let op = one_inp.parse::<Op>();
        assert_eq!(op, Ok(Op::Recover(Value::Register('a'))));

        let parsed = TEST_INPUT.lines()
            .map(|l| l.parse::<Op>().is_ok())
            .all(|o| o == true);
        assert!(parsed);
    }

    #[test]
    fn radix() {
        let p = 'p'.to_digit(36).unwrap();
        assert_eq!(p - 10, 15)
    }
}
