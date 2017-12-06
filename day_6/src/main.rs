use std::collections::HashSet;

fn main() {
    let mut input = "11	11	13	7	0	15	5	5	4	4	1	1	7	1	15	11"
        .split_whitespace()
        .map(|s| usize::from_str_radix(s, 10).unwrap())
        .collect::<Vec<_>>();

    let result = part_one(&mut input);
    println!("{}", result);

    let part_two = count_loop_cycles(input);
    println!("{}", part_two);

}

/// Part two. This code is.. :shrug:
fn count_loop_cycles(inp: Vec<usize>) -> usize {
    let mut loop_len = 0;
    let initial_layout = inp.clone();
    let mut current  = inp;

    loop {
        current = next_layout(&current);

        loop_len += 1;
        if current == initial_layout { break }
    }

    loop_len
}

fn next_layout(inp: &[usize]) -> Vec<usize> {
    let mut result = inp.to_owned();

    let idx = result.iter().enumerate().fold((0, 0), |min, cur| {
        if *cur.1 > min.1 {
            (cur.0, *cur.1)
        } else {
            min
        }
    }).0;

    let to_share = result[idx];
    result[idx] = 0;
    for i in 1..to_share + 1 {
        let idx = (idx + i) % result.len();
        result[idx] += 1;
    }
    result
}

fn part_one(inp: &mut [usize]) -> usize {
    let mut seen_states = HashSet::new();
    let mut num_ops = 0;

    loop {

        if !seen_states.insert(inp.to_owned()) { break }
        num_ops += 1;

        let idx = inp.iter().enumerate().fold((0, 0), |min, cur| {
            if *cur.1 > min.1 {
                (cur.0, *cur.1)
            } else {
                min
            }
        }).0;

        let to_share = inp[idx];
        inp[idx] = 0;
        for i in 1..to_share + 1 {
            let idx = (idx + i) % inp.len();
            inp[idx] += 1;
        }
    }
    num_ops
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let mut inp = vec![0, 2, 7, 0];
        assert_eq!(part_one(&mut inp), 5);
    }

    #[test]
    fn test_part_two() {
        let mut inp = vec![0, 2, 7, 0];
        part_one(&mut inp);
        let num_cycles = count_loop_cycles(inp);
        assert_eq!(num_cycles, 4);
    }
}
