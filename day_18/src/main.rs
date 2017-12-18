extern crate crossbeam;

use std::str::FromStr;
use std::sync::{mpsc, atomic, Arc};
use std::fmt::Error;

fn main() {
    let input = include_str!("../input.txt").trim()
        .lines()
        .map(|l| l.parse::<Op>().unwrap())
        .collect::<Vec<_>>();

    part_two(&input);
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
    pid: usize,
    ops: Vec<Op>,
    other_blocked: Arc<atomic::AtomicBool>,
    self_blocked: Arc<atomic::AtomicBool>,
    send_chan: mpsc::Sender<isize>,
    recv_chan: mpsc::Receiver<isize>,
    registers: [isize; 16],
    send_count: usize,
    run_state: RunState,
}

impl ProgramState {
    fn new(pid: usize, ops: &[Op],
           other_blocked: Arc<atomic::AtomicBool>,
           self_blocked: Arc<atomic::AtomicBool>,
           send: mpsc::Sender<isize>,
           recv: mpsc::Receiver<isize>) -> Self {

        let mut state =  ProgramState {
            pid: pid,
            ops: ops.to_owned(),
            other_blocked: other_blocked,
            self_blocked: self_blocked,
            send_chan: send,
            recv_chan: recv,
            registers: [0isize; 16],
            send_count: 0,
            run_state: RunState::Continue(0),
        };
        state.registers[15] = pid as isize;
        state
    }

    fn run(&mut self) -> Result<usize, String> {
        loop {
            let next_op = match self.run_state {
                RunState::Continue(idx) if idx <= self.ops.len() => idx,
                RunState::Halt => return Ok(self.send_count),
                RunState::Continue(idx) => return Err(format!("out of bounds: {}", idx)),
                RunState::OutOfBounds(idx) => return Err(format!("out of bounds: {}", idx)),
            };

            let op = self.ops[next_op];
            if let Err(e) = self.execute(&op) {
                eprintln!("proc {}, err {:}", self.pid, e);
                return Ok(self.send_count)
            }
        }
    }

    fn execute(&mut self, op: &Op) -> Result<(), String> {
        match op {
            &Op::Send(ref val) => {
                let send_val = self.get_value(val);
                self.send_count += 1;
                self.send_chan.send(send_val).map_err(|e| format!("{:?}", e))?;
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
            &Op::Receive(ref reg) => {
                let reg = reg.get_register().unwrap();
                let result = match self.recv_chan.try_recv() {
                    Ok(int) => Ok(int),
                    Err(_) => {
                        self.self_blocked.store(true, atomic::Ordering::SeqCst);
                        let result = if self.other_blocked.load(atomic::Ordering::SeqCst) {
                            self.recv_chan.try_recv().map_err(|_|
                                  format!("proc {}: other blocked, recv fail", self.pid))
                        } else {
                            self.recv_chan.recv().map_err(|_|
                               format!("proc {}: no block, recv fail", self.pid))
                        };
                        self.self_blocked.store(false, atomic::Ordering::SeqCst);
                        result
                    }
                };

                match result {
                    Ok(int) => {
                        self.set_reg(reg, int);
                        self.run_state.set_to_next();
                    }
                    Err(e) => {
                        eprintln!("proc {} END ERR {}", self.pid, e);
                        self.run_state = RunState::Halt;
                    }
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
        }
        Ok(())
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

fn part_two(ops: &[Op]) {

    let ops = ops.to_owned();
    crossbeam::scope(|scope| {

        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();
        let t1_blocked = Arc::new(atomic::AtomicBool::new(false));
        let t2_blocked = Arc::new(atomic::AtomicBool::new(false));
        let bf1 = t1_blocked.clone();
        let bf2 = t2_blocked.clone();

        scope.spawn(|| {
            let mut state1 = ProgramState::new(0, &ops, t1_blocked, bf2, tx1, rx2);
            state1.run();
        });
        let r = scope.spawn(||{
            let mut state2 = ProgramState::new(1, &ops, t2_blocked, bf1, tx2, rx1);
            state2.run()
        }).join();
        println!("p2: {:?}", r);
    })
}

type Register = char;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Value {
    Register(Register),
    Literal(isize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Send(Value),
    Set(Value, Value),
    Add(Value, Value),
    Mul(Value, Value),
    Mod(Value, Value),
    Receive(Value),
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
            "snd" => Ok(Op::Send(reg)),
            "set" => Ok(Op::Set(reg, val.unwrap())),
            "add" => Ok(Op::Add(reg, val.unwrap())),
            "mul" => Ok(Op::Mul(reg, val.unwrap())),
            "mod" => Ok(Op::Mod(reg, val.unwrap())),
            "rcv" => Ok(Op::Receive(reg)),
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
