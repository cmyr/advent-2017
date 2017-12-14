#![feature(slice_rotate)]

use std::fmt::Debug;
use std::ascii::AsciiExt;
use std::collections::HashSet;

fn main() {
    let input = "hxtvlmkl";
    let inp_vals: Vec<usize> = (0..256).collect();

    let p1 = part_one(input);
    println!("part one: {}", p1);

    // p2:
    let grid = hash_art(&input);
    let formatted = format_art(&grid);
    let p2 = part_two(input);
    println!("part two: {}", p2);
}

fn format_art(inp: &Vec<Vec<char>>) -> Vec<Vec<String>> {
    let mut out = Vec::new();
    for v in inp {
        let format_v = v.iter()
            .map(|c| if *c == '0' { ".".into() } else { "#".into() })
            .collect();
        out.push(format_v);
    }
    out
}

fn debug_print_grid(grid: &Vec<Vec<String>>) {
    for line in grid {
        println!("{}", line.iter().flat_map(|s| s.chars()).collect::<String>())
    }
}

fn part_one(input: &str) -> usize {
    let grid = hash_art(&input);
    grid.iter()
        .flat_map(|v| v.iter())
        .map(|c| if *c == '1' { 1 } else { 0 })
        .sum()
}

fn part_two(input: &str) -> usize {
    let grid = hash_art(&input);
    let mut grid = format_art(&grid);
    let mut seen = HashSet::new();
    let mut cur_group = 0;

    for i in 0..128 {
        for j in 0..128 {
            if grid[i][j] == "." { continue }
            if seen.contains(&(i, j)) { continue }
            cur_group += 1;
            recurs_find_neighbours(&mut grid, cur_group, (i, j), &mut seen);
        }
    }
    for line in grid {
        println!("{}", line.iter().flat_map(|s| s.chars()).collect::<String>())
    }
    cur_group
}

fn recurs_find_neighbours(grid: &mut Vec<Vec<String>>, cur_val: usize, cell: (usize, usize),
                          seen: &mut HashSet<(usize, usize)>) {
    if grid[cell.0][cell.1] == "." { return }
    if seen.contains(&cell) { return }
    seen.insert(cell);
    let deb_val = cur_val % 9;
    grid[cell.0][cell.1] = format!("{}", deb_val);
    let neighbours = neighbours_for_cell(cell);
    for n in neighbours {
        recurs_find_neighbours(grid, cur_val, n, seen);
    }
}

fn neighbours_for_cell(cell: (usize, usize)) -> Vec<(usize, usize)> {
    //let mut out = Vec::new();
    let i = cell.0 as isize;
    let j = cell.1 as isize;

    let neighbours = [
        (i - 1, j), (i + 1, j),
        (i, j - 1), (i, j + 1),
    ];

    neighbours.iter()
        .filter(|cell| cell.0 >= 0 && cell.1 >= 0 && cell.0 <= 127 && cell.1 <= 127)
        .map(|cell| (cell.0 as usize, cell.1 as usize))
        .collect()
}

fn hash_art(input: &str) -> Vec<Vec<char>> {
    let mut out = Vec::new();
    for i in 0..128 {
        let text_i = format!("{}-{}", input, i);
        let hash = final_knot_hash(&text_i);
        let hash = bitify_hex(&hash)
            .chars()
            .map(|c| c)
            .collect::<Vec<_>>();
        out.push(hash);
    }
    out
}

fn bitify_hex(hex_str: &str) -> String {
    hex_str.chars()
        .map(|b| format!("{:04b}", b.to_digit(16).unwrap()))
        .collect()
}


fn final_knot_hash(input: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bitify() {
        assert_eq!(bitify_hex("0"), "0000");
        assert_eq!(bitify_hex("1"), "0001");
        assert_eq!(bitify_hex("a0c2017"), "10100000110000100000000101110000")
    }

    #[test]
    fn test_part_one() {
        let test_inp = "flqrgnkx";
        let r = part_one(&test_inp);
        assert_eq!(r, 8108)
    }

    #[test]
    fn test_part_two() {
        let test_inp = "flqrgnkx";
        let r = part_two(&test_inp);
        assert_eq!(r, 1242);
    }
}
