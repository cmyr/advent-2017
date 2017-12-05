fn main() {
    let mut input: Vec<isize> = include_str!("../input.txt").lines()
        .map(str::trim)
        .map(|s| isize::from_str_radix(s, 10).expect("illegal input"))
        .collect();

    let part_1 = jump_to_escape(&mut input.clone(), |_| 1 );
    println!("{}", part_1);

    let part_2 = jump_to_escape(&mut input, |cur_val| {
        if cur_val >= 3 { -1 } else { 1 }
    });
    println!("{}", part_2);
}

/// Takes a closure as the second argument, which determines
/// how the value at the current position is modified.
fn jump_to_escape<F>(values: &mut [isize], offset_fn: F) -> isize
    where F: Fn(isize) -> isize
{
    let mut num_jumps = 0;
    let mut cur_idx = 0isize;
    while 0 <= cur_idx  && cur_idx < values.len() as isize {
        let next_idx = cur_idx + values[cur_idx as usize];
        values[cur_idx as usize] += offset_fn(values[cur_idx as usize]);
        cur_idx = next_idx;
        num_jumps += 1;
    }
    num_jumps
}
