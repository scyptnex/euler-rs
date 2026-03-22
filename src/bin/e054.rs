use euler_rs::res::get_input;
use itertools::Itertools;
use std::{fs::read_to_string, str::FromStr};

#[derive(Debug)]
struct Card {
    v: u32,
    s: char,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cs: Vec<char> = s.chars().collect();
        Ok(Card {
            v: match cs[0] {
                '0'..='9' => cs[0].to_digit(10).unwrap(),
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!(),
            },
            s: cs[1],
        })
    }
}

#[derive(Debug)]
struct Hand {
    h: Vec<Card>,
}

impl Hand {
    fn new(h: &[&str]) -> Self {
        assert!(h.len() == 5);
        Hand {
            h: h.iter().copied().flat_map(str::parse::<Card>).collect(),
        }
    }

    fn value(&self) -> u64 {
        let dscs = [
            Hand::high_card,
            Hand::pair,
            Hand::two_pair,
            Hand::triple,
            Hand::straight,
            Hand::flush,
        ];
        dscs.iter()
            .enumerate()
            .flat_map(|(i, d)| d(self).map(|v| (i, v)))
            .map(|(i, v)| ((i as u64) << 32) | (v as u64))
            .max()
            .unwrap()
    }

    fn flush(&self) -> Option<u32> {
        let mut it = self.h.iter();
        let suit = it.next().unwrap().s;
        if it.all(|c| c.s == suit) {
            self.high_card()
        } else {
            None
        }
    }

    fn straight(&self) -> Option<u32> {
        let mut sh = self.h.iter().map(|c| c.v).collect_vec();
        sh.sort();
        let mut it = sh.into_iter().enumerate();
        let start = it.next().unwrap().1;
        if it.all(|(i, v)| v == start + (i as u32)) {
            Some(start)
        } else {
            None
        }
    }

    fn triple(&self) -> Option<u32> {
        self.hgram()
            .iter()
            .filter(|kv| *kv.1 == 3)
            .map(|kv| *kv.0)
            .max()
    }

    fn two_pair(&self) -> Option<u32> {
        let mut v: Vec<_> = self.hgram().into_iter().filter(|kv| kv.1 == 2).collect();
        if v.len() != 2 {
            None
        } else {
            v.sort();
            Some(v[0].0 + 100 * v[1].0)
        }
    }

    fn pair(&self) -> Option<u32> {
        self.hgram()
            .iter()
            .filter(|kv| *kv.1 == 2)
            .map(|kv| *kv.0)
            .max()
    }

    fn hgram(&self) -> std::collections::HashMap<u32, usize> {
        let mut ret = std::collections::HashMap::<u32, usize>::new();
        for c in &self.h {
            *ret.entry(c.v).or_default() += 1
        }
        ret
    }

    fn high_card(&self) -> Option<u32> {
        self.h.iter().map(|c| c.v).max()
    }
}

fn solve() -> usize {
    read_to_string(get_input("0054_poker.txt"))
        .unwrap()
        .lines()
        .filter(|l| {
            let h = l.split(" ").collect::<Vec<_>>();
            let h1 = Hand::new(&h[..5]).value();
            let h2 = Hand::new(&h[5..]).value();
            dbg!(h1, h2);
            h1 > h2
        })
        .count()
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_straight() {
        assert!(Hand::new(&["3S", "4C", "5H", "6S", "7S"])
            .straight()
            .is_some());
        assert!(Hand::new(&["3S", "4C", "5H", "5S", "7S"])
            .straight()
            .is_none());
    }
}
