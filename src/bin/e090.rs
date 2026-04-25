use std::collections::HashSet;

use itertools::Itertools;

struct Dice {
    faces: HashSet<u64>,
}

impl Dice {
    fn new(d: Vec<u64>) -> Self {
        let mut faces = HashSet::from_iter(d.into_iter());
        if faces.contains(&6) {
            faces.insert(9);
        }
        if faces.contains(&9) {
            faces.insert(6);
        }
        Self { faces }
    }

    fn shows(&self, v: &u64) -> bool {
        self.faces.contains(v)
    }
}

fn is_squareable(d1: &Dice, d2: &Dice) -> bool {
    [
        (0, 1),
        (0, 4),
        (0, 9),
        (1, 6),
        (2, 5),
        (3, 6),
        (4, 9),
        (6, 4),
        (8, 1),
    ]
    .iter()
    .all(|(t, u)| (d1.shows(t) && d2.shows(u)) || (d1.shows(u) && d2.shows(t)))
}

fn count_squareable(d1: Dice) -> u64 {
    (0..=9)
        .combinations(6)
        .map(Dice::new)
        .filter(|d2| is_squareable(&d1, d2))
        .count() as u64
}

fn solve() -> u64 {
    (0..=9)
        .combinations(6)
        .map(Dice::new)
        .map(count_squareable)
        .sum::<u64>()
        / 2
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 1217);
    }
}
