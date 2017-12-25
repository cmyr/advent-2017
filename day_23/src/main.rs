#[macro_use]
extern crate lazy_static;

use std::str::FromStr;
use std::io::{self, Write};
use std::collections::HashSet;


// dear future archaeologist: this is a goddamn trainwreck

fn main() {
    let input = include_str!("../input.txt").trim()
        .lines()
        .map(|l| l.parse::<Op>().unwrap())
        .collect::<Vec<_>>();

    let raw_inp = include_str!("../input.txt").trim()
        .lines()
        .collect::<Vec<_>>();

    //let mut program = ProgramState::new(&input, &raw_inp);
    //let _ = program.run();
    //program.pretty_print();
    let p2 = optimized();
    println!("part two: {}", p2);
}

fn unoptimized() -> usize {
    let mut b = 106500;
    let c = 123500;
    let mut h = 0;

    'top: loop {
        let mut f = true; // 9
        let mut d = 2;
        'd: loop {
            let mut e = 2;
            'e: loop {
                let mut g = d; // 12
                g *= e;
                g -= b; // 14
                if g == 0 { f = false } // 15 / 16
                e += 1;
                g = e;
                g -= b;
                if g != 0 {
                    continue 'e
                } // 20
                d += 1;
                g = d;
                g -= b; // 23
                if g != 0 {
                    continue 'd
                }
                if !f {
                    h += 1; // 26
                }
                g = b;
                g -= c;
                if g == 0 {
                    return h
                }
                b += 17;
                continue 'top
            }
        }
    }
}

fn optimized() -> usize {
    let mut b = 106500;
    let c = 123500;
    let mut h = 0;
    loop {
        if is_bad_or_whatever(b) {
            h += 1;
        }
        if b == c {
            return h
        }
        b += 17;
    }
}

lazy_static! {
    static ref A_BUNCH_OF_NUMBERS: HashSet<usize> = {
        let mut m = HashSet::new();
        for i in 2..123500 {
            for j in 2..123500 {
                if i * j > 123500 { continue }
                m.insert(i*j);
            }
        }
        m
    };
}

fn is_bad_or_whatever(b: usize) -> bool {
    for i in A_BUNCH_OF_NUMBERS.iter() {
        if b - i == 0 { return true }
    }
    false
}

fn will_divide(b: usize) -> bool {
    let mut e = 2;
    while e < b {
        if b - (e * 2) == 0 {
            println!("{}", e);

            return true
        }
        e += 1;
    }
    return false
}
struct ProgramState {
    ops: Vec<Op>,
    raw_ops: Vec<&'static str>,
    registers: [isize; 8],
    mul_count: usize,
    cur_op: usize,
    prev_op: usize,
    run_count: usize,
}

impl ProgramState {
    fn new(ops: &[Op], raw_ops: &[&'static str]) -> Self {
        let mut registers = [0isize; 8];
        registers[0] = 1;
        ProgramState {
            ops: ops.to_owned(),
            raw_ops: raw_ops.to_owned(),
            registers: registers,
            mul_count: 0,
            cur_op: 0,
            prev_op: 0,
            run_count: 0,
        }
    }

    fn run(&mut self) -> Result<usize, String> {
        loop {
            self.get_input();
        }
    }

    fn step(&mut self, steps: usize) {
        for _run in 0..steps {
            self.prev_op = self.cur_op;
            if steps <= 100 {
                self.pretty_print();
            }
            if self.cur_op > 31 { break }
            if self.cur_op == 15 || self.cur_op == 24 || self.cur_op == 8 { self.pretty_print(); }
            //if self.cur_op == 18 {
                //print!("patching 18:");
                //self.pretty_print();
                //self.registers[6] = self.registers[1];
            //}
            //if self.cur_op == 18 || self.cur_op == 22 {
                //print!("patching {}: ", self.cur_op);
                //self.pretty_print();
                //self.registers[6] = self.registers[1];

            //}
            self.run_count += 1;
            let op = self.ops[self.cur_op];
            if let Err(_) = self.execute(&op) {
                break
            }
        }
        self.pretty_print();
    }

    fn get_input(&mut self) {
        let mut buf = String::new();
        let _ = io::stdin().read_line(&mut buf).unwrap();
        let cmd = buf.chars().next().unwrap_or(' ');
        match cmd {
            ' ' | '\n' => self.step(1),
            'r' => {
                let nb_runs = buf.trim_matches(|c: char| !c.is_digit(10)).parse::<usize>()
                    .unwrap();
                self.step(nb_runs);
            }
            'j' => {
                let op = buf.trim_matches(|c: char| !c.is_digit(10)).parse::<usize>()
                    .unwrap();
                self.cur_op = op;
            }
            's' => {
                let reg = buf.chars().nth(1).unwrap().to_digit(36).unwrap() as usize - 10;
                let val = buf.split_whitespace().nth(1).unwrap().parse::<isize>().unwrap();
                self.registers[reg] = val;
            }
            'p' => {
                self.pretty_print();
            }
            other => println!("unexpected command {}", other),
        }
    }


    fn pretty_print(&self) {
        println!("[ a: {:<7} b: {:<7} c: {:<7} d: {:<7} e: {:<7} f: {:<7} g: {:<7} h: {:<7}] {}: {} {}",
               self.registers[0], self.registers[1], self.registers[2], self.registers[3],
               self.registers[4], self.registers[5], self.registers[6], self.registers[7],
               self.prev_op + 1, self.raw_ops[self.prev_op],
               self.run_count);
    }

    fn execute(&mut self, op: &Op) -> Result<(), String> {
        match op {
            &Op::Set(ref reg, ref val) => {
                let reg = reg.get_register().unwrap();
                let val = self.get_value(val);
                self.set_reg(reg, val);
                self.cur_op += 1;
            }
            &Op::Sub(ref reg, ref val) => {
                let cur_val = self.get_value(reg);
                let reg = reg.get_register().unwrap();
                let val = self.get_value(val);
                self.set_reg(reg, cur_val - val);
                self.cur_op += 1;
            }
            &Op::Mul(ref reg, ref val) => {
                let cur_val = self.get_value(reg);
                let reg = reg.get_register().unwrap();
                let val = self.get_value(val);
                self.set_reg(reg, cur_val * val);
                self.cur_op += 1;
                self.mul_count += 1;
            }
            &Op::Jump(ref reg, ref val) => {
                let reg_val = self.get_value(reg);
                if reg_val != 0 {
                    let val = self.get_value(val);
                    self.jump(val);
                } else {
                    self.cur_op += 1;
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
        let next = self.cur_op as isize + offset;
        if next < 0 {
            self.cur_op = 33; // just some out of bounds value
        } else {
            self.cur_op = next as usize;
        }
    }
}

type Register = char;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Value {
    Register(Register),
    Literal(isize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Set(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
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
            "set" => Ok(Op::Set(reg, val.unwrap())),
            "sub" => Ok(Op::Sub(reg, val.unwrap())),
            "mul" => Ok(Op::Mul(reg, val.unwrap())),
            "jnz" => Ok(Op::Jump(reg, val.unwrap())),
            other => panic!("illegal instruction name '{}'", other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn primalness() {
        let some_primes = vec!{149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199};
        let not_primes = vec![150, 155, 166, 194, 195, 196];
        for p in some_primes {
            assert!(is_prime(p));
        }
        for p in not_primes {
            assert!(!is_prime(p));
        }
    }
}
