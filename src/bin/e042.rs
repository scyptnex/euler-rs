use euler_rs::infile;

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    let tris: std::collections::HashSet<u64> = (1..10_000).map(|n| (n * n + n) / 2).collect();
    infile!()
        .split(',')
        .map(|s| s.chars().filter(|c| *c != '"').collect::<String>())
        .map(|s| word_score(&s))
        .filter(|ws| tris.contains(ws))
        .count()
}

fn word_score(wrd: &str) -> u64 {
    wrd.bytes().map(|c| c - b'A' + 1).map(|b| b as u64).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(word_score("SKY"), 55);
    }
}
