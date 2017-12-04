use std::collections::BTreeMap;

fn main() {
    let input = 289326;
    // first version
    println!("{}", manhattan_distance(input));
    // new style:
    let coords = coords_for_idx(input as isize);
    println!("{}", coords.0.abs() + coords.1.abs());

    // part 2:
    let target_val = 289326_isize;
    let mut lookup = BTreeMap::new();
    let mut cur_val = 1;
    let mut i = 2;

    lookup.insert((0_isize, 0_isize), 1isize);
    while cur_val < target_val {
        let (x, y) = coords_for_idx(i);
        let neighbours = [(x, y-1), (x, y+1), (x+1, y+1), (x+1, y), (x+1, y-1),
                          (x-1, y+1), (x-1, y), (x-1, y-1)];
        cur_val = neighbours.iter()
            .map(|n| lookup.get(n).unwrap_or(&0))
            .sum();
        lookup.insert((x, y), cur_val);
        i += 1;
    }
     println!("{}", cur_val);
}

fn manhattan_distance(loc: usize) -> usize {
    let mut depth = 1usize;
    let mut side_len = 3usize;

    if loc == 1 { return 0 }

    loop {
        if side_len.pow(2) > loc { break }
        depth += 1;
        side_len += 2;
    }

    // index from start of this spiral
    let pos = loc - (side_len - 2).pow(2);
    // offset from the midpoint of this side
    let pos = pos % depth;

    pos as usize + depth
}

/// Given an index in 'spiral space', converts into coordinate space.
fn coords_for_idx(idx: isize) -> (isize, isize) {
    // the length of each side at this level in the spiral
    let mut h = (idx as f64).sqrt().ceil() as isize;
    if h % 2 == 0 { h+= 1 }

    // the max absolute x or y value for this index
    let w = (h - 1) / 2;
    // this index, offset from the start of this level in the spiral
    let rel_idx = idx - (h-2).pow(2);
    // which side are we on?
    let side = rel_idx / (w * 2);
    //eprintln!("idx {}, rel_idx {}, w {}, side: {}", idx, rel_idx, w, side);
    match side {
        0 => (w, rel_idx - w),
        1 => (w - (rel_idx - 2 * w), w),
        2 => (-w, w - (rel_idx - 4 * w)),
        3 => ((rel_idx - 6 * w) - w, -w),
        4 => (w, - w),
        _ => panic!("no pentagons allowed!"),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn manhattan() {
        assert_eq!(manhattan_distance(12), 3);
        assert_eq!(manhattan_distance(1), 0);
        assert_eq!(manhattan_distance(23), 2);
        assert_eq!(manhattan_distance(1024), 31);
    }

    #[test]
    fn to_coords() {
        assert_eq!(coords_for_idx(2), (1, 0));
        assert_eq!(coords_for_idx(4), (0, 1));
        assert_eq!(coords_for_idx(6), (-1, 0));
        assert_eq!(coords_for_idx(17), (-2, 2));
        assert_eq!(coords_for_idx(21), (-2, -2));
        assert_eq!(coords_for_idx(49), (3, -3));
        assert_eq!(coords_for_idx(31), (3, 3));
    }
}
