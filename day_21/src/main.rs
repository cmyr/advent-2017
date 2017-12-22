use std::str::FromStr;
use std::collections::HashMap;

fn main() {
    let inp = ".#./..#/###".parse::<Pattern>().unwrap();
    let rules = load_rules();
    let p1 = part_one(&inp, &rules, 5);
    println!("part one: {}", p1);
    let p2 = part_one(&inp, &rules, 7);
    println!("part two: {}", p2);
}

fn part_one(inp: &Pattern, rules: &HashMap<usize, Pattern>, nb_runs: usize) -> usize {
    let mut p = inp.to_owned();
    for _i in 0..nb_runs {
        p = p.next(&rules);
    }
    p.pattern.iter()
        .fold(0, |acc, p| acc + if *p { 1 } else { 0 })
}

fn load_rules() -> HashMap<usize, Pattern> {
    let raw_rules = include_str!("../input.txt").trim();
    let mut rulemap = HashMap::new();
    for line in raw_rules.lines() {
        let mut sides = line.split(" => ");
        let lhs: Pattern = sides.next().unwrap().parse().unwrap();
        let rhs: Pattern = sides.next().unwrap().parse().unwrap();
        rulemap.insert(lhs.fingerprint, rhs);
    }
    rulemap
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pattern {
    fingerprint: usize,
    pattern: Vec<bool>,
}

impl Pattern {
    fn next(&self, map: &HashMap<usize, Pattern>) -> Pattern {
        let subs = self.subpatterns();
        println!("current pattern: {}, {} subpatterns", self.pattern.len(), subs.len());
        self.pretty_print();
        let new_patterns = subs.iter()
            .map(|p| map.get(&p.fingerprint).unwrap().to_owned())
            .collect::<Vec<_>>();

        let new = if new_patterns.len() == 1 {
            new_patterns.first().unwrap().to_owned()
        } else {
            join_subs(&new_patterns)
        };
        new.pretty_print();
        new
    }

    fn pretty_print(&self) {
        let sl = (self.pattern.len() as f32).sqrt() as usize;
        println!("{:-^1$}", self.pattern.len(), sl+2);
        for i in 0..sl {
            let items = self.pattern[i*sl..i*sl+sl].iter()
                .map(|b| if *b {'#'} else {'.'})
                .collect::<String>();
            println!("{}", items);
        }
        let sl_real = (self.pattern.len() as f32).sqrt();
        assert_eq!(sl_real, sl_real.trunc(), "{}", sl);
    }

    fn subpatterns(&self) -> Vec<Pattern> {
        let sl = (self.pattern.len() as f32).sqrt() as usize;
        let sub_sl = if sl % 2 == 0 { 2 } else { 3 };
        if sub_sl == 3 { assert!(sl % 3 == 0, "{}", sl); }
        let nb_sub = sl / sub_sl;
        if nb_sub == 1 { return vec![self.clone()] }
        let mut out = Vec::new();
        for i in 0..nb_sub {
            for j in 0..nb_sub {
                out.push(make_sub(&self.pattern, sl, sub_sl, i, j));
            }
        }
        out
    }
}

fn make_sub(items: &[bool], side_len: usize, sub_len: usize, x: usize, y: usize) -> Pattern {
    let mut pattern = Vec::new();
    for i in 0..sub_len {
        let x = x * side_len * sub_len;
        let ii = x + i * side_len;
        for j in 0..sub_len {
            let y = y * sub_len;
            let jj = y + j;
            pattern.push(items[ii+jj]);
        }
    }

    let fingerprint = compute_fingerprint(&pattern);
    Pattern { pattern, fingerprint }
}

fn join_subs(subs: &[Pattern]) -> Pattern {
    let sl = (subs.len() as f32).sqrt() as usize;
    let pl = (subs.first().unwrap().pattern.len() as f32).sqrt() as usize;
    println!("joining {} subs, sl {}", subs.len(), sl);
    let mut out = Vec::new();
    for row_group in 0..sl {
        for i in 0..pl {
            for sub in &subs[row_group * sl..row_group * sl+sl] {
                let p = (sub.pattern.len() as f32).sqrt();
                //assert_eq!(p, p.trunc(), "{} {:?}", p, sub);
                let p = p as usize;
                out.extend(&sub.pattern[i * p..i * p + p]);
            }
        }
    }
    Pattern { pattern: out, fingerprint: 0 }
}

impl FromStr for Pattern {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = s.split('/')
            .flat_map(|r| r.chars().map(|c| if c == '.' { false } else { true }))
            .collect::<Vec<_>>();
        let fingerprint = compute_fingerprint(&pattern);
        Ok(Pattern { fingerprint, pattern })
    }
}

fn compute_fingerprint(pat: &[bool]) -> usize {
    let mut cur_score = 0;
    for p in permutations(&pat) {
        cur_score = cur_score.max(score_pattern(&p));
    }
    cur_score
}

fn score_pattern(pat: &[bool]) -> usize {
    let sl = (pat.len() as f32).sqrt() as usize;
    let mut score = sl << 16 ;
    for (i, b) in pat.iter().enumerate() {
        if *b {
            score = score ^ 1 << i;
        }
    }
    score
}

struct Permutations {
    input: Vec<bool>,
    current_transform: u8,
}

impl Permutations {
    fn new(pat: &[bool]) -> Self {
        Permutations {
            input: pat.to_owned(),
            current_transform: 0,
        }
    }

    fn rotate(&mut self) {
        rotate(&mut self.input);
    }

    fn mirror(&mut self) {
        mirror(&mut self.input);
    }
}

impl Iterator for Permutations {
    type Item = Vec<bool>;

    fn next(&mut self) -> Option<Vec<bool>> {
        let result = match self.current_transform {
            0 => Some(self.input.clone()),
            // rotations
            1 | 3 | 5 | 7 => {
                self.mirror();
                let result = self.input.clone();
                self.mirror();
                Some(result)
            }
            2 | 4 | 6 => {
                self.rotate();
                Some(self.input.clone())
            }
            8 => None,
            _ => panic!("transforms greater than 8 should be unreachable"),
        };
        self.current_transform += 1;
        result
    }

}

fn permutations(pat: &[bool]) -> Permutations {
    Permutations::new(pat)
}

fn rotate(pat: &mut [bool]) {
    let mut temp = [false; 16];
    let sl = (pat.len() as f32).sqrt() as usize;
    for i in 0..sl {
        for j in 0..sl {
            let idx = i * sl + j;
            let new_idx = ((sl - 1) - j) * sl + i;
            temp[idx] = pat[new_idx];
        }
    }
    let len = pat.len();
    for i in 0..len {
        pat[i] = temp[i];
    }
}

fn mirror(pat: &mut [bool]) {
    let mut temp = [false; 16];
    let sl = (pat.len() as f32).sqrt() as usize;
    for i in 0..sl {
        for j in 0..sl {
            let idx = i * sl + j;
            let new_idx = i * sl + ((sl - 1) - j);
            temp[idx] = pat[new_idx];
        }
    }

    let len = pat.len();
    for i in 0..len {
        pat[i] = temp[i];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rotation() {
        let mut p = Permutations::new(&[true, false, false, false]);
        let mut inp = p.input.clone();
        rotate(&mut inp);
        assert_eq!(inp[1], true, "{:?}", inp);
        rotate(&mut inp);
        //assert_eq!(inp[1], false);
        assert_eq!(inp[1], false, "{:?}", inp);
        assert_eq!(inp[3], true, "{:?}", inp);


        let mut p = Permutations::new(
            &[false, true, false,
            false, false, true,
            true, true, true]);

        let mut inp = p.input.clone();
        rotate(&mut inp);
        let exp = [
            true, false, false,
            true, false, true,
            true, true, false];
        assert_eq!(&inp, &exp);
    }

    #[test]
    fn scores() {
        let inp = vec![false, true, true, false];
        assert_eq!(score_pattern(&inp), 6);
        let inp = vec![true, true, true, false];
        assert_eq!(score_pattern(&inp), 7);
        let inp = vec![true, true, true, true];
        assert_eq!(score_pattern(&inp), 15);
    }
}
