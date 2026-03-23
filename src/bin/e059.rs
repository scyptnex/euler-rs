use euler_rs::res::get_input;
use itertools::Itertools;
use std::{collections::HashSet, fs::read_to_string};

fn decipher(ct: &Vec<u8>, k: [u8; 3]) -> Option<String> {
    let v: Vec<u8> = ct.iter().enumerate().map(|(i, b)| b ^ k[i % 3]).collect();
    if v.iter().any(|b| !b.is_ascii()) {
        None
    } else {
        Some(String::from_utf8_lossy(&v).to_string())
    }
}

fn score(pt: &str, engl: &HashSet<&str>) -> usize {
    pt.split_whitespace().filter(|w| engl.contains(w)).count()
}

fn keyspace() -> impl Iterator<Item = [u8; 3]> {
    (b'a'..=b'z')
        .cartesian_product(b'a'..=b'z')
        .cartesian_product(b'a'..=b'z')
        .map(|k| [k.0 .0, k.0 .1, k.1])
}

fn solve() -> usize {
    let engl = HashSet::from(["a", "the", "you", "me", "for", "to"]);
    let cipher = read_to_string(get_input("0059_cipher.txt"))
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u8>().unwrap())
        .collect_vec();
    keyspace()
        .flat_map(|k| decipher(&cipher, k))
        .max_by_key(|pt| score(&pt.to_lowercase(), &engl))
        .unwrap()
        .bytes()
        .map(|b| b as usize)
        .sum()
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(
            decipher(&b"555555".iter().copied().collect_vec(), [1, 2, 3]).unwrap(),
            "476476"
        );
        assert_eq!(solve(), 129448);
    }
}
