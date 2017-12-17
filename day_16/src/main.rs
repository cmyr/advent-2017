#![feature(slice_rotate)]
use std::str::FromStr;
use std::fmt;
use std::mem;
use std::time::{Instant, Duration};


fn main() {
    let input = include_str!("../input.txt").trim()
        .split(',')
        .map(|s| s.parse::<Op>().unwrap())
        .collect::<Vec<_>>();

    let state = "abcdefghijklmnop".chars().collect::<Vec<_>>();
    let p1 = part_one(&input, &state, 1);
    println!("part one {}", p1.iter().collect::<String>());

    let ONE_BILLLLLLION = 1_000_000_000;
    let nb_ops = ONE_BILLLLLLION % 60;
    let mut state = ['a', 'b', 'c', 'd', 'e','f','g','h','i','j','k','l','m','n','o','p'];

    let p2 = part_one(&input, &state, nb_ops);

    println!("part two {}", p2.iter().collect::<String>());

}

fn part_one(ops: &[Op], state: &[char], runs: usize) -> Vec<char> {
    let mut state = state.to_owned();
    let initial_state = state.clone();
    for _i in 0..runs {
        for op in ops {
            op.operate(&mut state);
            //op.debug();
            if _i % 1000 == 0 {
                //println!("{}", _i);
            }
            //println!("{:?} {:?}", state, op);
        }
        if state == initial_state {
            println!("repeated at {}", _i);
        }
    }
    state
}


// wrong path
fn make_map(end_state: &[char]) -> Vec<usize> {
    // for each char, and each index, find the starting index
    // create a new vec, where for the value at each index is the final
    // index of the char that started there.
    let mut original_order = end_state.to_owned();
    let mut result = Vec::new();
    original_order.sort();
    for c in end_state.iter() {
        let end_idx = original_order.iter().position(|el| el == c).unwrap();
        result.push(end_idx);
    }
    result
}

// unused, ultimately
fn part_two(ops: &[Op], state: &[char], runs: usize) -> Vec<char> {
    let mut sample_end_state = state.to_owned();
    let mut state = state.to_owned();

    for op in ops {
        op.operate(&mut sample_end_state);
    }

    assert!(sample_end_state != state);
    let op_map = make_map(&sample_end_state);

    let mut new_state = state.clone();
    for _i in 0..runs {
        //if _i % 100_000 == 0 { println!("i: {}", _i) }
        for j in 0..state.len() {
            //new_state[j] = state[*op_map.get(&j).unwrap()];
            new_state[j] = state[op_map[j]];
        }
        mem::swap(&mut state, &mut new_state);
        //state = new_state;
    }
    state
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl Op {
    fn operate(&self, state: &mut [char]) {
        let s_len = state.len();
        match *self {
            Op::Spin(n) => state.rotate(s_len - n),
            Op::Exchange(one, two) => {
                let temp = state[one];
                state[one] = state[two];
                state[two] = temp;
            }
            Op::Partner(one, two) => {
                let idx_one = state.iter().position(|c| c == &one).unwrap();
                let idx_two = state.iter().position(|c| c == &two).unwrap();
                state[idx_one] = two;
                state[idx_two] = one;
            }
        }
    }
}

impl FromStr for Op {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (op, rest) = s.split_at(1);
		match op {
			"s" => Ok(Op::Spin(rest.parse().map_err(|e| format!("{}", e))?)),
			"x" => {
                let mut nums = rest.split('/')
                    .map(|n| n.parse::<usize>().unwrap());
                Ok(Op::Exchange(nums.next().unwrap(), nums.next().unwrap()))
            }
            "p" => {
                let mut rest = rest.chars();
                let p1 = rest.next().unwrap();
                let _slash = rest.next();
                let p2 = rest.next().unwrap();
                Ok(Op::Partner(p1, p2))
            }
            _ => panic!("unexpected input {}", s),
        }
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parsing() {
        let inp1 = "s1";
        let inp2 = "x3/14";
        let inp3 = "pe/b";
        assert_eq!(inp1.parse::<Op>().unwrap(), Op::Spin(1));
        assert_eq!(inp2.parse::<Op>().unwrap(), Op::Exchange(3, 14));
        assert_eq!(inp3.parse::<Op>().unwrap(), Op::Partner('e', 'b'));
    }

    #[test]
    fn p1_test() {
        let mut inp = "abcde".chars().collect::<Vec<_>>();
        Op::Spin(1).operate(&mut inp);
        assert_eq!(inp, vec!['e', 'a', 'b', 'c', 'd']);
        Op::Exchange(3, 4).operate(&mut inp);
        assert_eq!(inp, vec!['e', 'a', 'b', 'd', 'c']);
        Op::Partner('e', 'b').operate(&mut inp);
        assert_eq!(inp, vec!['b', 'a', 'e', 'd', 'c']);
    }
}
