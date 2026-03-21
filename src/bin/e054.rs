use euler_rs::res::get_input;
use std::fs::read_to_string;

fn hand(h: &[&str]) -> u64 {
    assert!(h.len() == 5);
    println!("{:?}", h);
    0
}

fn solve() -> usize {
    read_to_string(get_input("0054_poker.txt"))
        .unwrap()
        .lines()
        .filter(|l| {
            let h = l.split(" ").collect::<Vec<_>>();
            hand(&h[..5]) > hand(&h[5..])
        })
        .count()
}

fn main() {
    println!("{}", solve());
}
