//! Advent of Code, day 1

#![feature(test)]
extern crate test;

fn main() {
    let input = include_str!("../input.txt").trim();
    let input: Vec<char> = input.chars().collect();
    let result = sum_matching_next(&input);
    println!("{}", result);

    let p2_result = sum_matching_opposite(&input);
    println!("{}", p2_result);
}

/// Day 1, Exercise 1
///
/// Returns the sum of all digits in the series which are equal to
/// the next digit in the series, wrapping around.
fn sum_matching_next(series: &[char]) -> u32 {

    if series.len() <= 1 { return 0 }

    let mut sum = 0;
    let mut peekable_series = series.iter().peekable();

    while let Some(i) = peekable_series.next() {
        sum += match peekable_series.peek() {
            Some(next) if next == &i => i.to_digit(10).unwrap(),
            // None means end of series; check against first item
            None if Some(i) == series.first() => i.to_digit(10).unwrap(),
            _ => 0,
        };
    }
    sum
}

/// Day 1, Exercise 2
///
/// Returns the sum of all digits in the series for which the digit
/// at position i + n/2 is equal to the digit at posiition `i`, where
/// `n` is the length of the series.
///
/// # Panics
///
/// This function will panic if the length of the input is non-even.
fn sum_matching_opposite(series: &[char]) -> u32 {
    assert!(series.len() % 2 == 0, "input series must be even length");

    let offset = series.len() / 2;

    series.iter().enumerate().fold(0, |sum, (idx, i)| {
        // modulo to wrap around
        let opp_idx = (idx + offset) % series.len();
        sum + if &series[opp_idx] == i { i.to_digit(10).unwrap() } else { 0 }
    })
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

    #[test]
    fn test_part_two() {
        let inp = "1212".chars().collect::<Vec<_>>();
        assert_eq!(sum_matching_opposite(&inp), 6);

        let inp = "1221".chars().collect::<Vec<_>>();
        assert_eq!(sum_matching_opposite(&inp), 0);

        let inp = "123425".chars().collect::<Vec<_>>();
        assert_eq!(sum_matching_opposite(&inp), 4);

        let inp = "123123".chars().collect::<Vec<_>>();
        assert_eq!(sum_matching_opposite(&inp), 12);

        let inp = "12131415".chars().collect::<Vec<_>>();
        assert_eq!(sum_matching_opposite(&inp), 4);
    }

    #[bench]
    fn bench_sum_matching_next(b: &mut Bencher) {
        let input = include_str!("../input.txt").trim();
        let input: Vec<char> = input.chars().collect();
        b.iter(|| sum_matching_next(&input) );
    }
}
