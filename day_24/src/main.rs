fn main() {
    let pieces = parse_input();
    //let p1 = best_span(pieces.clone(), 0, 0, 0);
    //println!("part one: {}", p1);
    let best = best_span(pieces.clone(), 0, 0, 0);
    println!("part two: {:?}", best);
}

fn best_span(pieces: Vec<(usize, usize)>, slot: usize, score: usize, length: usize) -> (usize, usize) {
    let candidates = pieces.iter()
        .enumerate()
        .filter(|&(_, p)| p.0 == slot || p.1 == slot)
        .map(|(i, _)| i);

    let mut best = (score, length);
    for i in candidates {
        let mut pieces = pieces.clone();
        let candidate = pieces.remove(i);
        let our_score = score + candidate.0 + candidate.1;
        let new_slot = if candidate.0 == slot { candidate.1 } else { candidate.0 };
        let our_score = best_span(pieces, new_slot, our_score, length + 1);
        println!("{:?}", &our_score);
        if our_score.1 > best.1 || (our_score.1 == best.1 && our_score.0 > best.0) {
            best = our_score;
        }
        //best = best.max(our_score);
    }
    best
}

fn parse_input() -> Vec<(usize, usize)> {
    include_str!("../input.txt").trim()
        .lines()
        .map(|l| {
            let mut iter = l.split('/');
            (iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap())
        })
        .collect()
}
