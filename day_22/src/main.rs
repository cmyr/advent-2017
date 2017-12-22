use std::collections::{HashSet, HashMap};
use std::iter;

fn main() {
    let inp = include_str!("../input.txt");
    let infected = load_grid(inp);
    sanity_check(&infected);

    let p1 = part_one(&infected);
    println!("part one {}", p1);

    let p2 = part_two(&infected);
    println!("part two {}", p2);
}

fn part_one(infected: &HashSet<Coord>) -> usize {
    let mut infected = infected.to_owned();
    let mut infections = 0;
    let mut position = Coord { x: 0, y: 0 };
    let mut direction = Direction::Up;
    for _run in 0..10_000 {
        if infected.contains(&position) {
            direction = direction.to_right();
            infected.remove(&position);
        } else {
            direction = direction.to_left();
            infected.insert(position.clone());
            infections += 1;
        }
        position = position.to_direction(&direction);
    }
    infections

}

fn part_two(infected: &HashSet<Coord>) -> usize {
    let mut infected: HashMap<Coord, NodeState> = infected.iter()
        .cloned()
        .zip(iter::repeat(NodeState::Infected))
        .collect();
    let mut infections = 0;
    let mut position = Coord { x: 0, y: 0 };
    let mut direction = Direction::Up;
    for _run in 0..10_000_000 {
        let mut remove = false;
        let exists = infected.contains_key(&position);
        if exists {
            let state = infected.get_mut(&position).unwrap();
            direction = match *state {
                NodeState::Infected => direction.to_right(),
                NodeState::Flagged => direction.to_right().to_right(),
                NodeState::Weakened => direction,
                NodeState::Clean => unreachable!(),
            };
            state.transition();
            remove = state.is_clean();
            if state.is_infected() {
                infections += 1;
            }
        } else {
            infected.insert(position, NodeState::Weakened);
            direction = direction.to_left();
        }
        if remove {
            infected.remove(&position);
        }
        position = position.to_direction(&direction);
    }
    infections
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl NodeState {
    fn transition(&mut self) {
        let new_state = match *self {
            NodeState::Clean => NodeState::Weakened,
            NodeState::Weakened => NodeState::Infected,
            NodeState::Infected => NodeState::Flagged,
            NodeState::Flagged => NodeState::Clean,
        };
        *self = new_state;
    }

    fn is_clean(&self) -> bool {
        match *self {
            NodeState::Clean => true,
            _ => false,
        }
    }

    fn is_infected(&self) -> bool {
        match *self {
            NodeState::Infected => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn to_left(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
    fn to_right(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl Coord {
    fn to_direction(&self, d: &Direction) -> Coord {
        match *d {
            Direction::Up => Coord { x: self.x, y: self.y - 1 },
            Direction::Down => Coord { x: self.x, y: self.y + 1 },
            Direction::Left => Coord { x: self.x - 1, y: self.y },
            Direction::Right => Coord { x: self.x + 1, y: self.y },
        }
    }
}

fn load_grid(inp: &str) -> HashSet<Coord> {
    let inp = inp.trim()
        .lines()
        .map(|l| l.chars().map(|c| if c == '#' { true } else { false }).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let height = inp.len();
    let width = inp[0].len();
    let y_bias = height / 2;
    let x_bias = width / 2;
    let mut infected = HashSet::new();
    for (i, row) in inp.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col {
                let x = j as isize - x_bias as isize;
                let y = i as isize - y_bias as isize;
                infected.insert(Coord { x, y });
            }
        }
    }
    infected
}

fn sanity_check(infected: &HashSet<Coord>) {
    assert!(!infected.contains(&Coord { x: -12, y: -12 }));
    assert!(infected.contains(&Coord { x: -12, y: 12 }));
    assert!(!infected.contains(&Coord { x: 12, y: 12 }));
    assert!(infected.contains(&Coord { x: -12, y: 12 }));
    assert!(!infected.contains(&Coord { x: -12, y: 11 }));
    assert!(infected.contains(&Coord { x: -11, y: 12 }));
    assert!(!infected.contains(&Coord { x: -10, y: 12 }));
    assert!(infected.contains(&Coord { x: 0, y: 0 }));
    assert!(infected.contains(&Coord { x: 1, y: 0 }));
    assert!(!infected.contains(&Coord { x: -1, y: 0 }));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_two_test() {
        let inp = r#"..#
#..
..."#;
        let grid = load_grid(inp);
        let p2 = part_two(&grid);
        assert_eq!(p2, 2511944);
    }
}
