fn main() {
    let input = include_str!("../input.txt").trim()
        .lines()
        .map(parse)
        .collect::<Vec<_>>();

    let p1 = part_one(&input);
    println!("part one: {}", p1);

    let p2 = part_two(&input);
    println!("part two: {}", p2);
}

fn parse(line: &str) -> (usize, usize) {
    let mut iter = line.split_whitespace()
        .map(|el| str::parse::<usize>(el.trim_matches(':')).unwrap());
    (iter.next().unwrap(), iter.next().unwrap())
}

#[derive(Debug, Clone)]
struct Board {
    steps: Vec<Option<Layer>>,
}

impl Board {
    fn new(layers: &[(usize, usize)]) -> Self {
        let mut steps = Vec::with_capacity(layers.last().unwrap().1);
        for &(idx, range) in layers {
            while idx > steps.len() {
                steps.push(None)
            }
            steps.push(Some(Layer { depth: idx, range: range, scanner_pos: 0, advancing: true }))
        }
        Board { steps }
    }

    fn score_for_pos(&self, pos: usize) -> usize {
        match self.steps[pos] {
            Some(ref layer) if layer.scanner_pos == 0 => layer.depth * layer.range,
            _ => 0
        }
    }

    fn seen_at_pos(&self, pos: usize) -> bool {
        match self.steps[pos] {
            Some(ref layer) if layer.scanner_pos == 0 => true,
            _ => false
        }
    }

    fn step(&mut self) {
        self.steps.iter_mut()
            .for_each(|l| { l.as_mut().map(|l| l.step()); } )
    }
}

#[derive(Debug, Clone)]
struct Layer {
    depth: usize,
    range: usize,
    scanner_pos: usize,
    advancing: bool,
}

impl Layer {
    fn step(&mut self) {
        let new_pos = if self.advancing { self.scanner_pos + 1 } else { self.scanner_pos - 1 };
        if new_pos == 0 || new_pos == self.range -1 { self.advancing = !self.advancing }
        self.scanner_pos = new_pos;
    }
}

fn part_one(input: &[(usize, usize)]) -> usize {
    let mut board = Board::new(input);
    let mut cur_pos = 0;
    let mut score = 0;
    while cur_pos < board.steps.len() {
        // a timestep
        // we ignore the first move, it's a noop
        score += board.score_for_pos(cur_pos);
        board.step();
        cur_pos += 1;

    }
    score
}

/// this is... bad
fn part_two(input: &[(usize, usize)]) -> usize {
    let mut big_board = Board::new(input);
    let mut offset_time = 0;
    'outer: loop {
        big_board.step();
        offset_time += 1;
        let mut board = big_board.clone();
        let mut cur_pos = 0;
        while cur_pos < board.steps.len() {
            if board.seen_at_pos(cur_pos) {
                //if cur_pos > 30 {
                    //println!("offset {} seen at {}", offset_time, cur_pos);
                //}
                continue 'outer
            }
            board.step();
            cur_pos += 1;
        }
        return offset_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_make() {
        let inp = vec![(0, 3), (4, 1)];
        let board = Board::new(&inp);
        assert_eq!(board.steps.len(), 5);
        assert!(board.steps[2].is_none());
        assert!(board.steps[4].is_some());
    }

    #[test]
    fn test_part_two() {
        let inp = vec![(0, 3), (1, 2), (4, 4), (6, 4)];
        let r = part_two(&inp);
        assert_eq!(r, 10);
    }

    #[test]
    fn hit_at() {
        let inp = vec![(0, 3), (1, 2), (4, 4), (6, 4)];
        let mut board = Board::new(&inp);
        let hit_at = steps_until_hit(&mut board);
        assert_eq!(hit_at, 0);
    }
}
