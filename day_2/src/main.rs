
fn main() {
    let input = include_str!("../input.txt");

    let result_1 = max_gap_checksum(&input.trim());
    println!("{}", result_1);

    let result_2 = divisor_checksum(&input.trim());
    println!("{}", result_2);
}


/// Given a string containing whitespace-delineated decimal numbers,
/// returns the sum of the difference of the min and max number on
/// for each line.
fn max_gap_checksum(spreadsheet: &str) -> usize {
    spreadsheet.split('\n')
        .map(|line| {
            line.split_whitespace()
                .fold((usize::max_value(), 0usize), |(min, max), item| {
                    let item = usize::from_str_radix(item, 10).unwrap();
                    (min.min(item), max.max(item))
                })
        })
    .map(|(min, max)| max - min)
    .sum()
}

/// Given a string containing whitespace-delineated decimal numbers,
/// finds the two numbers `(a, b)` in each line for which `a` is a divisor
/// of b, returning the sum of `b / a` for each line.
fn divisor_checksum(spreadsheet: &str) -> usize {
    spreadsheet.split('\n')
        .map(|line| {
            let  line: Vec<usize> = line.split_whitespace()
                .map(|num| usize::from_str_radix(num, 10).unwrap())
                .collect();

            // we might have a very small win if we sorted the line here
            pair_in_line(&line)
                .expect("line does not contain factorable pair")

        })
    .sum()
}


/// Given a sequence of numbers, attempts to find a pair `(a, b)`
/// in the sequence for which `a` is a divisor of `b`, and returns
/// their product.
///
/// Note: this function is quadratic.
fn pair_in_line(line: &[usize]) -> Option<usize> {
    for i in line {
        for j in line {
            if i == j { continue }
            if j % i == 0 {
                return Some(j / i)
            }
        }
    }
    None
}
