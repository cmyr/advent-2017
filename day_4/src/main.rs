use std::collections::HashSet;

fn main() {
    let input: Vec<_> = include_str!("../input.txt").trim()
        .lines()
        .collect();

    let num_valid: usize = input.iter()
        .map(|line| if contains_dupe_words(line) { 0 } else { 1 })
        .sum();
    println!("{}", num_valid);

    let num_valid: usize = input.iter()
        .map(|line| if contains_anagram_words(line) { 0 } else { 1 })
        .sum();
    println!("{}", num_valid);
}

fn contains_dupe_words(line: &'static str) -> bool {
    let mut seen = HashSet::new();
    for word in line.split_whitespace() {
        if !seen.insert(word) {
            return true
        }
    }
    false
}

fn contains_anagram_words(line: &'static str) -> bool {
    let mut seen = HashSet::new();
    for word in line.split_whitespace() {
        let mut chars: Vec<char> = word.chars().collect();
        // two anagramed words are identical when sorted
        chars.sort();

        if !seen.insert(chars) {
            return true
        }
    }
    false
}
