#![feature(test)]

extern crate test;

fn main() {
    let input = include_str!("../input.txt").trim();
    let input: Vec<char> = input.chars().collect();
    let result = sum_matching_next(&input);
    println!("{}", result);
}

fn sum_matching_next(series: &[char]) -> u32 {

    if series.len() <= 1 {
        return 0
    }

    let mut sum = 0;
    let mut peekable_series = series.iter().peekable();

    while let Some(i) = peekable_series.next() {
        sum += match peekable_series.peek() {
            Some(next) if next == &i => i.to_digit(10).unwrap(),
            None if Some(i) == series.first() => i.to_digit(10).unwrap(),
            _ => 0,
        };
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_samples() {
        let inp = "91212129".chars().collect::<Vec<_>>();
        assert_eq!(sum_matching_next(&inp), 9);

        let inp = "1111".chars().collect::<Vec<_>>();
        assert_eq!(sum_matching_next(&inp), 4);

        let inp = "1234".chars().collect::<Vec<_>>();
        assert_eq!(sum_matching_next(&inp), 0);

        let inp = "1122".chars().collect::<Vec<_>>();
        assert_eq!(sum_matching_next(&inp), 3);
    }

    #[bench]
    fn bench_sum_matching_next(b: &mut Bencher) {
        let input = include_str!("../input.txt").trim();
        let input: Vec<char> = input.chars().collect();
        b.iter(|| sum_matching_next(&input) );
    }
}
