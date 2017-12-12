use std::collections::{BTreeSet, BTreeMap, BinaryHeap};
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;

fn main() {
    let input = include_str!("../input.txt").trim()
        .lines()
        .map(parse)
        .collect::<Vec<_>>();

    let p1 = part_one(&input);
    println!("part one: {}", p1);

    let p2 = part_two(&input);
    println!("part one: {}", p2);
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    ident: usize,
    neighbours: BTreeSet<usize>,
    visited: bool,
    group: Option<usize>,
}

struct RefNode(Rc<RefCell<Node>>);

impl PartialEq for RefNode {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().eq(&other.0.borrow())
    }
}

impl Eq for RefNode {}

impl Ord for RefNode {
    fn cmp(&self, other: &RefNode) -> Ordering {
        other.0.borrow().ident.cmp(&self.0.borrow().ident)
    }
}

impl PartialOrd for RefNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.borrow().ident.partial_cmp(&self.0.borrow().ident)
    }
}


impl Clone for RefNode {
    fn clone(&self) -> Self {
        RefNode(self.0.clone())
    }
}

impl RefNode {
    fn new(n: Node) -> Self {
        RefNode(Rc::new(RefCell::new(n)))
    }
}


fn parse(line: &str) -> Node {
    let mut iter = line.split_whitespace();
    let first = iter.next().map(str::parse::<usize>).unwrap().unwrap();
    let _ = iter.next();
    let rest = iter.map(|s| s.trim_matches(',').parse::<usize>().unwrap()).collect();
    Node { ident: first, neighbours: rest, visited: false, group: None }
}

fn part_one(nodes: &[Node]) -> usize {
    let nodes = nodes.iter()
        .map(|n| (n.ident, RefNode::new(n.clone())))
        .collect::<BTreeMap<_, _>>();

    let first = nodes.get(&0).unwrap().clone();
    let mut heap = BinaryHeap::new();
    heap.push(first);

    let mut count = 0;

    while let Some(next) = heap.pop() {
        if next.0.borrow().visited { continue }
        next.0.borrow_mut().visited = true;
        count += 1;

        let next_borrow = next.0.borrow();
        let to_visit = next_borrow.neighbours.iter()
            .map(|n| nodes.get(n).unwrap().clone())
            .filter(|n| n.0.borrow().visited == false);
        heap.extend(to_visit);
    }

    count
}

fn part_two(nodes: &[Node]) -> usize {
    let nodes = nodes.iter()
        .map(|n| (n.ident, RefNode::new(n.clone())))
        .collect::<BTreeMap<_, _>>();

    let mut cur_group = 0;

    let node_ids = nodes.keys().cloned().collect::<Vec<_>>();
    for id in node_ids {
        let node = nodes.get(&id).unwrap();
        if node.0.borrow().group.is_some() { continue }
        cur_group += 1;
        let mut heap = BinaryHeap::new();
        heap.push(node);
        while let Some(next) = heap.pop() {
            next.0.borrow_mut().group = Some(cur_group);

            let next_borrow = next.0.borrow();
            let to_visit = next_borrow.neighbours.iter()
                .map(|n| nodes.get(n).unwrap())
                .filter(|n| n.0.borrow().group.is_none());
            heap.extend(to_visit);
        }
    }

    cur_group
}
