use std::collections::{BinaryHeap, HashSet};
use std::str::FromStr;
use std::cmp::Ordering;


fn main() {
    let input = include_str!("../input.txt").trim()
        .split(',')
        .map(|e| e.parse::<Direction>().unwrap())
        .collect::<Vec<_>>();

    let path = build_path(&input);
    let p1 = shortest_path(path[0].position, path.last().unwrap().position);
    println!("part one: {}", p1);

    let p2 = furthest_point(&path);
    println!("part one: {}", p2);
}

type Position = (isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    position: Position,
    edges: [Position; 6],
    cost: usize,
}

fn build_path(directions: &[Direction]) -> Vec<Node> {
    // start with a node at (0, 0)
    // track nodes in a map
    // at each step, create the node if needed
    // stop with a big list of nodes?
    let mut nodes = Vec::new();
    let mut cur_pos = (0, 0);
    let mut cur_node = Node::new(cur_pos, 0);
    nodes.push(cur_node);

    for direction in directions {
        cur_pos = cur_node.relative_position(*direction);
        cur_node = Node::new(cur_pos, 0);
        nodes.push(cur_node);
    }
    nodes
}

fn shortest_path(pos: Position, target: Position) -> usize {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();
    heap.push(Node::new(pos, 0));
    while let Some(next) = heap.pop() {
        //println!("visiting {:?}", next);
        if next.position.0 > 1000 { break }
        if seen.contains(&next.position) { continue }
        if next.position == target { return next.cost }
        seen.insert(next.position);
        let cur_dist = position_distance(next.position, target);
        let to_visit = next.edges.iter()
            .filter(|e| !seen.contains(e))
            .filter(|e| position_distance(**e, target) <= cur_dist)
            .map(|e| Node::new(*e, next.cost + 1));
        heap.extend(to_visit);
    }
    unreachable!()
}

fn furthest_point(path: &[Node]) -> usize {
    let start = path.first().unwrap();
    let mut max_dist = 0;
    for node in path {
        let dist = node.distance_from(start);
        max_dist = max_dist.max(dist);
    }

    max_dist
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Node {
    fn new(position: Position, cost: usize) -> Self {
        let edges = [
            (position.0, position.1 + 2),
            (position.0 + 1, position.1 + 1),
            (position.0 + 1, position.1 - 1),
            (position.0, position.1 - 2),
            (position.0 - 1, position.1 - 1),
            (position.0 - 1, position.1 + 1),
        ];

        Node { position, edges, cost }
    }

    fn relative_position(&self, direction: Direction) -> Position {
        self.edges[direction as usize]
    }

    fn distance_from(&self, other: &Node) -> usize {
        position_distance(self.position, other.position)
    }
}

fn position_distance(p1: Position, p2: Position) -> usize {
    let mut norm_x = (p1.0 - p2.0).abs() as usize;
    let mut norm_y = (p1.1 - p2.1).abs() as usize;
    let diag = norm_y.min(norm_x);
    norm_x -= diag;
    norm_y -= diag;
    if norm_x == norm_y { return diag }
    if norm_x == 0 { return diag + norm_y / 2 }
    if norm_y == 0 { return diag + norm_x }
    unreachable!()
}

impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "n" => Ok(Direction::North),
            "ne" => Ok(Direction::NorthEast),
            "nw" => Ok(Direction::NorthWest),
            "s" => Ok(Direction::South),
            "se" => Ok(Direction::SouthEast),
            "sw" => Ok(Direction::SouthWest),
            other => Err(format!("unexpected direction {}", other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn distance() {
        assert_eq!(position_distance((0, 0), (0, 2)), 1);
        assert_eq!(position_distance((0, 0), (0, -8)), 4);
        assert_eq!(position_distance((0, 0), (-6, 0)), 6);
        assert_eq!(position_distance((0, 0), (-3, -1)), 3);
        assert_eq!(position_distance((0, 0), (3, 7)), 5);
    }
}
