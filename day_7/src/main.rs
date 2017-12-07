use std::collections::{HashSet, HashMap};
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Cow;

type CowStr = Cow<'static, str>;

fn main() {
    let input = include_str!("../input.txt").lines().collect::<Vec<_>>();
    let root = find_root(&input);
    println!("part one: {}", root);

    let tree = build_tree(&input);
    let bad_weight = find_imbalance(&tree)
        .expect("no imbalance found");
    println!("part two {}", bad_weight);
}

/// A node in a tree.
///
/// Note: This doesn't actually need the `RefCell`; in an earlier version
/// of this solution cells needed to be mutated at a certain point.
struct Node {
    name: CowStr,
    weight: usize,
    children: Vec<Rc<RefCell<Node>>>,
    calc_weight: usize,
}

/// Parses an input line into a (name, weight, [child_name]) tuple.
fn parse_line(line: &'static str) -> (CowStr, usize, Vec<CowStr>) {
    let mut iter = line.split_whitespace();
    let name = iter.next().unwrap();
    let size = iter.next()
        .map(|val|
             usize::from_str_radix(val.trim_matches(|c: char| !c.is_numeric()), 10)
             .unwrap()
             )
        .unwrap();

    // discard the -> field
    let _ = iter.next();
    let children = iter.map(|v| v.trim_matches(',').into()).collect::<Vec<_>>();
    (name.into(), size, children)
}

/// Parses input lines and constructs a tree.
fn build_tree<'a>(items: &[&'static str]) -> Rc<RefCell<Node>> {
    let nodes = items.iter().map(|l| {
        let item = parse_line(*l);
        (item.0.clone(), item)
    })
    .collect::<HashMap<_, _>>();
    let head = find_root(items);
    make_node(head, &nodes)
}

/// Given the name of the head node and a map of names -> node tuples,
/// constructs the nodes and builds the tree.
fn make_node(head: CowStr,
             nodes: &HashMap<CowStr, (CowStr, usize, Vec<CowStr>)>)
             -> Rc<RefCell<Node>> {
    let tup = nodes.get(&head).unwrap();

    let children: Vec<Rc<RefCell<Node>>> = tup.2.iter()
        .map(|c| make_node(c.clone(), nodes)).collect();

    let calc_weight: usize = children.iter()
        .fold(tup.1, |sum, c| sum + c.borrow().calc_weight);

    Rc::new(RefCell::new(
        Node {
            name: tup.0.clone(),
            weight: tup.1,
            children: children,
            calc_weight: calc_weight,
        }))
}


/// Given a slice of node tuples, returns the name of a node with
/// no parents.
///
/// The caller is responsible for ensuring that the input is well formed;
/// specifically there should be a single root node, and no node should
/// have multiple parents.
fn find_root<'a>(items: &[&'static str]) -> CowStr {
    let mut children = HashSet::new();
    let mut nodes = HashSet::new();

    for line in items {
        let (name, _, kids) = parse_line(line);
        nodes.insert(name);
        kids.iter().for_each(|c| { children.insert(c.clone()); });
    }
    nodes.difference(&children).next().unwrap().to_owned()
}

/// The part two solution.
fn find_imbalance(node: &Rc<RefCell<Node>>) -> Option<usize> {
    // count the number of distinct weights at this level in the tree.
    // There should be only 1 or 2, so this isn't especially elegant.
    let mut weights = HashMap::new();
    for child in &node.borrow().children {
        if let Some(prev) = weights.insert(child.borrow().calc_weight, 1) {
            weights.insert(child.borrow().calc_weight, prev + 1);
        }
    }

    // If there is more than a single distinct weight at this level,
    // we're in the section of tree that contains the error.
    if weights.len() > 1 {
        // do some sanity checking on our inputs
        assert!(weights.len() == 2, format!("{:?}", weights));
        assert!(node.borrow().children.len() > 2,
        "we expect more than two children in imbalanced tree segment?");

        // the bad child is the only child whose weight occurs just once.
        // this is why we assert len(2) above, because this is a big assumption.
        let bad_child = node.borrow().children.iter()
            .find(|c| weights.get(&c.borrow().calc_weight) == Some(&1))
            .map(|c| c.clone())
            .unwrap();

        let bad_weight = bad_child.borrow().calc_weight;

        // check children to see if _they're_ the problem
        // This isn't very expensive; if we're the problem then our children
        // won't recurse themselves
        for child in &bad_child.borrow().children {
            if let Some(result) = find_imbalance(&child.clone()) {
                return Some(result)
            }
        }

        // if we've gotten this far, we're the problem. Compare our calculated
        // weight against our sibling weights, and return the difference.
        let sibl_weight = *weights.keys()
            .find(|w| **w != bad_weight)
            .unwrap();

        if bad_weight > sibl_weight {
            Some(bad_child.borrow().weight - (bad_weight - sibl_weight))
        } else {
            Some(bad_child.borrow().weight + (sibl_weight - bad_weight))
        }
    } else {
        None
    }
}

/// Prints the tree
#[allow(dead_code)]
fn print_tree(head: &Rc<RefCell<Node>>, level: usize) {
    for child in &head.borrow().children {
        print_tree(&child, level + 1);
    }
    println!("{:0$} {2:>1$} {3:} {4:}", level, 15-level, head.borrow().name, head.borrow().calc_weight, head.borrow().weight);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let inp = "gzxnn (171) -> mqodhde, djvkd";
        assert_eq!(parse_line(inp), ("gzxnn".into(), 171,
        vec!["mqodhde".into(), "djvkd".into()]));
        let inp = "nljmcv (47)";
        assert_eq!(parse_line(inp), ("nljmcv".into(), 47, vec![]));
    }

    #[test]
    fn part_two() {
    let input = r#"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"#.lines().collect::<Vec<_>>();

    let tree = build_tree(&input);
    let bad_weight = find_imbalance(&tree);
    assert_eq!(bad_weight, Some(60));
    }
}
