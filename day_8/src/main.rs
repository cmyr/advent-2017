use std::collections::HashMap;

type Register = String;
type Instruction = (Register, isize, Condition);

fn main() {
    let input = include_str!("../input.txt").trim().lines().collect::<Vec<_>>();
    let instructions = input.iter().map(|line| parse_line(&line)).collect::<Vec<_>>();
    let result = part_both(&instructions);
    println!("part one: {}", result.0);
    println!("part two: {}", result.1);

}

/// Returns both the max final value and the max value at any point.
fn part_both(instructions: &[Instruction]) -> (isize, isize) {
    // use a hashmap for registers so we can keep them as strings
    let mut registers = HashMap::<Register, isize>::new();
    let mut highest_seen = 0;

    for instruction in instructions {
        let condition = &instruction.2;
        let lhs = *registers.get(&condition.register).unwrap_or(&0);
        if condition.comparison.compare(lhs, condition.other_side) {
            let cur_val = *registers.get(&instruction.0).unwrap_or(&0);
            let new_val = cur_val + instruction.1;
            if new_val > highest_seen { highest_seen = new_val }
            registers.insert(instruction.0.clone(), new_val);
        }
    }
    (*registers.values().max().unwrap_or(&0), highest_seen)
}

#[derive(Debug, Clone, PartialEq)]
enum Op {
    Equal,
    GreaterThanOrEqual,
    GreaterThan,
    LessThan,
    LessThanOrEqual,
    NotEqual,
}

impl<'a> From<&'a str> for Op {
    fn from(src: &'a str) -> Op {
        match src {
            "==" => Op::Equal,
            "!=" => Op::NotEqual,
            ">" => Op::GreaterThan,
            ">=" => Op::GreaterThanOrEqual,
            "<" => Op::LessThan,
            "<=" => Op::LessThanOrEqual,
            other => panic!("unexpected op: {}", other),
        }
    }
}

impl Op {
    fn compare(&self, lhs: isize, rhs: isize) -> bool {
        match *self {
            Op::Equal => lhs == rhs,
            Op::NotEqual => lhs != rhs,
            Op::GreaterThan => lhs > rhs,
            Op::GreaterThanOrEqual => lhs >= rhs,
            Op::LessThan => lhs < rhs,
            Op::LessThanOrEqual => lhs <= rhs,
        }
    }
}

struct Condition {
    register: Register,
    comparison: Op,
    other_side: isize,
}

/// Parses an input line into a (name, weight, [child_name]) tuple.
fn parse_line(line: &'static str) -> (Register, isize, Condition) {
    let mut iter = line.split_whitespace();
    let register = iter.next().unwrap();
    let inc = iter.next().unwrap();
    let mut val = isize::from_str_radix(iter.next().unwrap(), 10).unwrap();
    if inc == "dec" { val *= -1 }

    // discard the if field
    assert_eq!(iter.next(), Some("if"));
    let cond_reg = iter.next().unwrap();
    let cond_op = Op::from(iter.next().unwrap());
    let cond_num = isize::from_str_radix(iter.next().unwrap(), 10).unwrap();
    let cond = Condition { register: cond_reg.into(), comparison: cond_op, other_side: cond_num };
    (register.into(), val, cond)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let inp = "c dec -10 if a >= 1";
        let result = parse_line(&inp);
        assert_eq!(result.0, String::from("c"));
        assert_eq!(result.1, 10);
        assert_eq!(result.2.register, String::from("a"));
        assert_eq!(result.2.comparison, Op::GreaterThanOrEqual);
        assert_eq!(result.2.other_side, 1);
    }
}
