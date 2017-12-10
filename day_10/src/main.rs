#![feature(slice_rotate)]

use std::fmt::Debug;
use std::ascii::AsciiExt;

fn main() {
    let input = "165,1,255,31,87,52,24,113,0,91,148,254,158,2,73,153";
    let inp_vals: Vec<usize> = (0..256).collect();

    let p1 = part_one(input);
    println!("part one: {}", p1[0] * p1[1]);

    // p2:
    let p2 = part_two(input);
    println!("part two: {}", p2);
}

fn part_one(input: &str) -> Vec<usize> {
    let lengths = input.split(',')
        .map(|v| usize::from_str_radix(v, 10).unwrap())
        .collect::<Vec<_>>();

    let inp_vals: Vec<usize> = (0..256).collect();
    knot_hash(&inp_vals, &lengths, 0, 0).data
}

fn part_two(input: &str) -> String {
    assert!(input.is_ascii());
    let mut lengths: Vec<usize> = input.bytes().map(|v| v as usize).collect();
    let mut salt = vec![17, 31, 73, 47, 23];
    lengths.append(&mut salt);
    let mut val: Vec<usize> = (0..256).collect();

    let mut cur_pos = 0;
    let mut skip_size = 0;
    for _ in 0..64 {
        let result = knot_hash(&val, &lengths, skip_size, cur_pos);
        val = result.data;
        cur_pos = result.final_pos;
        skip_size = result.skip_size;
    }

    // reduce the result
    val.chunks(16).map(|chk| chk.iter().fold(0, |ac, i| ac ^ i))
        .inspect(|v| assert!(*v <= 255))
        .map(to_hex)
        .collect::<String>()
}

fn to_hex<T: Into<usize>>(i: T) -> String {
    format!("{:02x}", i.into())
}

struct KnotResult<T> {
    data: Vec<T>,
    final_pos: usize,
    skip_size: usize,
}

fn knot_hash<T: Copy + Debug>(input: &[T],
                              lengths: &[usize],
                              skip_size: usize,
                              start_pos: usize) -> KnotResult<T> {
    let mut input = input.to_owned();
    let mut skip_size = skip_size;
    let mut cur_pos = start_pos;
    let input_len = input.len();

    for len in lengths {
        let start = cur_pos % input_len ;
        let end = (cur_pos + len) % input_len;
        if end <= start {
            input.rotate(start);
            {
                let sub_v = &mut input[0..*len];
                sub_v.reverse();
            }
            input.rotate(input_len - start);
        } else {
            assert!(end != start, "end == start, what do to?");
            let sub_v = &mut input[start..end];
            sub_v.reverse();
        }
        cur_pos += len + skip_size;
        skip_size += 1;
    }
        let data = input;
        let final_pos = cur_pos;
        KnotResult { data, final_pos, skip_size }
}

fn smoke_test() {
    let sample_input = vec![0, 1, 2, 3, 4];
    let sample_lens = vec![3, 4, 1, 5];
    let r = knot_hash(&sample_input, &sample_lens, 0, 0).data;
    println!("debug result: {}", r[0] * r[1]);

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(""), String::from("a2582a3a0e66e6e86e3812dcb672a272"));
    }

    #[test]
    fn test_hex_gen() {
        assert_eq!(to_hex(2u8), String::from("02"));
        assert_eq!(to_hex(42u8), String::from("2a"));
    }
}
