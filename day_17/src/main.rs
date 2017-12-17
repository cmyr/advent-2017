
static INPUT: usize = 312;

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
    assert!(state.len() == 2018);
    println!("part one {}", result);
}

fn part_two() {
    let mut state = vec![0];
    let mut cur_pos = 0;
    for i in 0..50_000_000 {
        let i = i + 1;
        cur_pos = next_insert_idx(INPUT, cur_pos, state.len());
        // we only bother actually inserting the value if it's at idx 1;
        // other values we can just push.
        if cur_pos == 1 {
            if state.len() == 1 {
                state.push(i);
            } else {
                state.insert(1, i);
            }
        } else {
            state.push(i);
        }
    }

    let zero_pos = state.iter().position(|el| *el == 0).unwrap();
    let result = state[zero_pos+1];
    println!("part two {}", result);
}

fn spin_insert(spin_count: usize,
               to_insert: usize,
               cur_pos: usize,
               state: &mut Vec<usize>) -> usize {
    // find the next insertion point:
    let next_idx = next_insert_idx(spin_count, cur_pos, state.len());
    if next_idx > state.len() {
        assert!(next_idx == state.len() + 1);
        state.push(to_insert);
    } else {
        state.insert(next_idx, to_insert);
    }
    next_idx
}

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
