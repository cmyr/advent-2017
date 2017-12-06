use std::collections::HashSet;

fn main() {
    let input = "11	11	13	7	0	15	5	5	4	4	1	1	7	1	15	11"
        .split_whitespace()
        .map(|s| usize::from_str_radix(s, 10).unwrap())
        .collect::<Vec<_>>();

    let result = day_six(input.clone());
    println!("{}", result.0);
    println!("{}", result.1);

}


fn day_six(mut inp: Vec<usize>) -> (usize, usize) {
    let mut seen_states = HashSet::new();
    let mut num_ops = 0;

    // part two
    let mut in_loop = false;
    let mut loop_len = 1;
    let mut loop_start: Option<Vec<usize>> = None;

    loop {
        if in_loop == true {
            if loop_start.as_ref() == Some(&inp) {
                return (num_ops - loop_len, loop_len)
            } else {
                loop_len += 1;
            }
        }
        if !in_loop && !seen_states.insert(inp.clone()) {
            in_loop = true;
            loop_start = Some(inp.clone());
        }

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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let inp = vec![0, 2, 7, 0];
        assert_eq!(day_six(inp), (5, 4));
    }
}
