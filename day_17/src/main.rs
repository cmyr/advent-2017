
const INPUT: usize = 312;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let mut state = vec![0];
    let mut cur_pos = 0;
    for i in 0..2017 {
        let i = i + 1;
        cur_pos = spin_insert(INPUT, i, cur_pos, &mut state);
    }
    let last_insert = state.iter().position(|el| *el == 2017).unwrap();
    let result = state[last_insert+1];
    println!("part one {}", result);
}

/// For part two, we don't really care about the state at all;
/// we just care about whatever item ends at index 1.
fn part_two() {
    let mut cur_pos = 0;
    let mut cur_result = 0;

    for i in 1..50_000_001 {
        cur_pos = next_insert_idx(INPUT, cur_pos, i);
        if cur_pos == 1 {
            cur_result = i;
        }
    }
    println!("part two {}", cur_result);
}

fn spin_insert(spin_count: usize,
               to_insert: usize,
               cur_pos: usize,
               state: &mut Vec<usize>) -> usize {
    let next_idx = next_insert_idx(spin_count, cur_pos, state.len());
    if next_idx > state.len() {
        state.push(to_insert);
    } else {
        state.insert(next_idx, to_insert);
    }
    next_idx
}

#[inline(always)]
fn next_insert_idx(spin_count: usize, cur_pos: usize, cur_len: usize) -> usize {
    ((cur_pos + spin_count) % cur_len) + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finding_index() {
        assert_eq!(next_insert_idx(3, 0, 1), 1);
        assert_eq!(next_insert_idx(3, 1, 2), 1);
        assert_eq!(next_insert_idx(3, 2, 3), 1);
    }
}
