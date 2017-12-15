fn main() {
    let p1 = part_one();
    println!("part one: {}", p1);

    let p2 = part_two();
    println!("part two: {}", p2);
}

static A_FACTOR: usize = 16807;
static B_FACTOR: usize = 48271;

fn part_one() -> usize {
    let mut cur_a = 883;
    let mut cur_b = 879;
    let mut count = 0;

    for _i in 0..40_000_000 {
        let next_a = next_value(cur_a, A_FACTOR);
        let next_b = next_value(cur_b, B_FACTOR);
        if low_16_match(next_a, next_b) { count += 1 }
        cur_a = next_a;
        cur_b = next_b;
    }
    count
}


fn part_two() -> usize {
    let cur_a = 883;
    let cur_b = 879;
    let gen_a = ValueGenerator::new(cur_a, A_FACTOR);
    let gen_b = ValueGenerator::new(cur_b, B_FACTOR);
    gen_a.filter(|v| v % 4 == 0).zip(gen_b.filter(|v| v % 8 == 0))
        .filter(|&(v1, v2)| low_16_match(v1, v2))
        .count()
}

fn next_value(prev_val: usize, factor: usize) -> usize {
    let divisor = 2147483647;
    (prev_val * factor) % divisor
}

fn low_16_match(n1: usize, n2: usize) -> bool {
    n1 % 2usize.pow(16) == n2 % 2usize.pow(16)
}

struct ValueGenerator {
    cur_val: usize,
    nb_runs: usize,
    constant_factor: usize,
}

impl ValueGenerator {
    fn new(cur_val: usize, constant_factor: usize) -> Self {
        let nb_runs = 0;
        ValueGenerator { cur_val, nb_runs, constant_factor }
    }
}

impl Iterator for ValueGenerator {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.nb_runs == 40_000_000 { return None }
        let next_val = next_value(self.cur_val, self.constant_factor);
        self.nb_runs += 1;
        self.cur_val = next_val;
        Some(next_val)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_vals() {
        assert_eq!(next_value(65, 16807), 1092455);
        assert_eq!(next_value(1092455, 16807), 1181022009);
    }

    #[test]
    fn bin_match() {
        let v1 = 245556042;
        let v2 = 1431495498;
        assert!(low_16_match(v1, v2));
    }
}
