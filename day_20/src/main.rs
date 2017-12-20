use std::str::FromStr;
use std::collections::HashSet;
use std::io;
use std::io::Write;

fn main() {
    let input = include_str!("../input.txt").trim()
        .lines()
        .map(|l| l.parse::<Particle>().unwrap())
        .collect::<Vec<_>>();

    let p1 = part_one(&input);
    println!("part one: {}", p1);

    let p2 = part_two(&input);
    println!("part two: {}", p2);
}

fn part_one(particles: &[Particle]) -> usize {
    let mut particles = particles.to_owned();
    for _i in 0..10_000 {
        particles.iter_mut()
            .for_each(|p| p.to_next_state())
    }
    let min = particles.iter().enumerate()
        .min_by_key(|&(i, p)| p.distance_from_origin()).unwrap();
    min.0
}

fn part_two(particles: &[Particle]) -> usize {
    let mut particles = particles.to_owned();
    for _i in 0..200 {
        run_removing_collisions(&mut particles, 1000);
        print!("{}\r", _i * 1000);
        io::stdout().flush();
    }
    particles.len()
}

fn run_removing_collisions(particles: &mut Vec<Particle>, runs: usize) {
    let mut positions = HashSet::new();
    let mut collisions = HashSet::new();
    let mut to_remove = Vec::new();
    for _i in 0..runs {
        collisions.clear();
        particles.iter_mut().for_each(Particle::to_next_state);
        for p in particles.iter() {
            if !positions.insert(p.position) {
                collisions.insert(p.position);
            }
        }
        if !collisions.is_empty() {
            for (i, p) in particles.iter().enumerate() {
                if collisions.contains(&p.position) {
                    to_remove.push(i);
                }
            }
            positions.clear();

            for i in to_remove.iter().rev() {
                eprintln!("removing {}", i);
                particles.remove(*i);
            }

            to_remove.clear();
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Particle {
    position: Position,
    velocity: Velocity,
    acceleration: Acceleration,
}

impl Particle {
    fn to_next_state(&mut self) {
        let next = self.next_state();
        self.velocity.x += self.acceleration.x;
        self.velocity.y += self.acceleration.y;
        self.velocity.z += self.acceleration.z;

        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn distance_from_origin(&self) -> isize {
        self.position.x.abs() +
            self.position.y.abs() +
            self.position.z.abs()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
    z: isize,
}

type Position = Coord;
type Velocity = Coord;
type Acceleration = Coord;

impl FromStr for Coord {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim_matches(|c: char| !(c.is_digit(10) || c == ',' || c =='-'))
            .split(',')
            .map(|sub| sub.parse::<isize>().unwrap());
        Ok(Coord {
            x: iter.next().unwrap(),
            y: iter.next().unwrap(),
            z: iter.next().unwrap(),
        })
    }
}

impl FromStr for Particle {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(", ")
            .map(|sub_s| sub_s.trim().parse::<Coord>().unwrap());
        Ok(Particle {
            position: iter.next().unwrap(),
            velocity: iter.next().unwrap(),
            acceleration: iter.next().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_test() {
        let t = "<-6474,1279,-4265>";
        let c = t.parse::<Coord>();
        assert_eq!(c, Ok(Coord { x: -6474, y: 1279, z: -4265 }));
        let t2 = "p=<-1622,66,2201>, v=<-230,14,319>, a=<12,6,-21>";
        let c2 = t2.parse::<Particle>();
        assert!(c2.is_ok());
    }
}

