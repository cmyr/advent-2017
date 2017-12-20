use std::iter;

fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

   let (one, two) = part_both(&Map(input));
   println!("part one: {}", one.iter().collect::<String>());
   println!("part two: {}", two);
}

struct Map(Vec<Vec<char>>);

impl Map {
    fn tile_for_coord(&self, coord: &Coord) -> Option<char> {
        match self.0[coord.1][coord.0] {
            ' ' => None,
            other => Some(other),
        }
    }

    fn tile_in_direction(&self, coord: &Coord, direction: &Direction) -> Option<char> {
        let coord = coord.in_direction(direction);
        if coord.0 >= self.0[0].len() || coord.1 >= self.0.len() { return None }
        self.tile_for_coord(&coord)
    }
}

#[derive(Debug, Clone, Copy)]
struct Coord(usize, usize);

impl Coord {
    fn in_direction(&self, d: &Direction) -> Coord {
        match d {
            &Direction::Up => Coord(self.0, self.1.saturating_sub(1)),
            &Direction::Down => Coord(self.0, self.1 + 1),
            &Direction::Left => Coord(self.0.saturating_sub(1), self.1),
            &Direction::Right => Coord(self.0 + 1, self.1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn crosses(&self) -> [Direction; 2] {
        use Direction::*;
        match *self {
            Up | Down => [Left, Right],
            Left | Right => [Up, Down]
        }
    }
}

fn part_both(map: &Map) -> (Vec<char>, usize) {
    let start_x = map.0[0].iter().position(|el| *el == '|').unwrap();
    let mut pos = Coord(start_x, 0);
    let mut direction = Direction::Down;
    let mut letters = Vec::new();
    let mut steps = 0;
    loop {
        //let cur_pos = *&pos;
        let cur_tile = match map.tile_for_coord(&pos) {
            Some(c) => c,
            None => return (letters, steps),
        };

        match cur_tile {
            '|' | '-' => pos = pos.in_direction(&direction),
            l @ 'a' ... 'z' | l @ 'A' ... 'Z' => {
                letters.push(l);
                pos = pos.in_direction(&direction);
            }
            '+' => {
                let cur_d = direction;
                for d in direction.crosses().iter().chain(iter::once(&cur_d)) {
                    if let Some(_) = map.tile_in_direction(&pos, d) {
                        pos = pos.in_direction(d);
                        direction = *d;
                        break
                    }
                }
            }
            other => panic!("unexpected char {}", other),
        }
        steps += 1;

        //println!("({}, {}) -> ({}, {}), {} {:?}",
        //cur_pos.0, cur_pos.1, pos.0, pos.1, cur_tile, direction);
    }

}
