use euler_rs::infile;

fn main() {
    println!("{}", solve());
}

fn solve() -> u64 {
    let mut names: Vec<String> = infile!()
        .split(',')
        .map(|s| s.chars().filter(|c| *c != '"').collect())
        .collect();
    names.sort();
    names
        .iter()
        .enumerate()
        .map(|(i, n)| (i as u64 + 1) * score(n))
        .sum()
}

fn score(n: &str) -> u64 {
    n.bytes().map(|c| (c + 1 - b'A') as u64).sum()
}
