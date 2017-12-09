fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Hello, world!");

    let result_one = part_one(&input);
    println!("part one {}", result_one);

    let result_two = part_two(&input);
    println!("part two {}", result_two);
}


#[derive(Debug, Clone)]
/// The current parsing state
enum State {
    Ready,
    Group,
    Garbage,
    Escape,
}

#[derive(Debug, Clone)]
/// Transition operations
enum Op {
    Push(State),
    Pop,
    Continue,
}

impl State {
    fn transition(&self, c: char) -> Result<Op, String> {
        match *self {
            State::Ready => {
                match c {
                    '{' => Ok(Op::Push(State::Group)),
                    _ => Err(format!("unexpected char '{}' in state {:?}", c, self)),
                }
            }
            State::Group => {
                match c {
                    '{' => Ok(Op::Push(State::Group)),
                    '<' => Ok(Op::Push(State::Garbage)),
                    '}' => Ok(Op::Pop),
                    ',' => Ok(Op::Continue),
                    _ => Err(format!("unexpected char '{}' in state {:?}", c, self)),
                }
            }
            State::Garbage => {
                match c {
                    '>' => Ok(Op::Pop),
                    '!' => Ok(Op::Push(State::Escape)),
                    _ => Ok(Op::Continue),
                }
            }
            State::Escape => Ok(Op::Pop),
        }
    }

    fn is_group(&self) -> bool {
        match *self {
            State::Group => true,
            _ => false,
        }
    }

    fn is_garbage(&self) -> bool {
        match *self {
            State::Garbage => true,
            _ => false,
        }
    }
}

/// Parse the stream one character at a time; increasing and decreasing
/// the level tally with each group push/pop.
fn part_one(input: &'static str) -> usize {
    let mut stack = Vec::new();
    let mut score = 0;
    let mut level = 0;
    for (i, c) in input.chars().enumerate() {
        match stack.last().unwrap_or(&State::Ready).transition(c) {
            Ok(Op::Push(state)) => {
                if state.is_group() {
                    level += 1;
                    score += level;
                }
                stack.push(state);
            }
            Ok(Op::Pop) => {
                let prev_state = stack.pop().unwrap();
                if prev_state.is_group() {
                    level -= 1;
                }
            }
            Ok(Op::Continue) => continue,
            Err(msg) => eprintln!("Error at char {}: {}", i, msg),
        }
    }
    score
}

/// Ditto part one, but here we only count occurances of `Op::Continue`
/// while we're in a garbage group.
fn part_two(input: &'static str) -> usize {
    let mut stack = Vec::new();
    let mut garbage = 0;
    for (i, c) in input.chars().enumerate() {
        let is_garbage = stack.last().unwrap_or(&State::Ready).is_garbage();
        match stack.last().unwrap_or(&State::Ready).transition(c) {
            Ok(Op::Push(state)) => {
                stack.push(state);
            }
            Ok(Op::Pop) => {
                stack.pop();
            }
            Ok(Op::Continue) => {
                if is_garbage {
                    garbage += 1;
                }
                continue
            }
            Err(msg) => eprintln!("Error at char {}: {}", i, msg),
        }
    }
    garbage
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        assert_eq!(part_one("{}"), 1);
        assert_eq!(part_one("{{{}}}"), 6);
        assert_eq!(part_one("{{},{}}"), 5);
        assert_eq!(part_one("{{{},{},{{}}}}"), 16);
        assert_eq!(part_one("{<a>,<a>,<a>,<a>}"), 1);
        assert_eq!(part_one("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
        assert_eq!(part_one("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
        assert_eq!(part_one("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("{<{o\"i!a,<{i<a>}"), 10);
    }
}
