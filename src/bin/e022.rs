use euler_rs::res::get_input;
use std::fs::read_to_string;

fn main() {
    println!("{}", solve());
}

fn solve() -> u64 {
    let mut names: Vec<String> = read_to_string(get_input("0022_names.txt"))
        .unwrap()
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
