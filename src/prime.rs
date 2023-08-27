use std::cmp::max;
use std::iter::{repeat, Iterator};

/// Get the list of primes below |below|.
///
/// Example:
/// ```
/// use euler_rs::prime::sieve;
/// let primes = sieve(11);
/// assert_eq!(primes, vec![2,3,5,7]);
/// ```
pub fn sieve(below: usize) -> Vec<u64> {
    Sieve::new(2, below - 2).exclude_sift()
}

struct Sieve {
    start: u64,
    masks: Vec<bool>,
}

impl Sieve {
    fn new(start: u64, size: usize) -> Self {
        Sieve {
            start,
            masks: repeat(true).take(size).collect(),
        }
    }

    fn sift(&self) -> Vec<u64> {
        self.masks
            .iter()
            .enumerate()
            .filter(|(_, v)| **v)
            .map(|(i, _)| i as u64 + self.start)
            .collect()
    }

    fn exclude(&mut self, prime: u64) {
        let len = self.masks.len();
        for x in (max(2, self.start / prime)..)
            .map(|m| m * prime)
            .filter(|v| *v >= self.start)
            .map(|v| v - self.start)
            .map(|v| v as usize)
            .take_while(|v| *v < len)
        {
            self.masks[x] = false;
        }
    }

    fn exclude_sift(&mut self) -> Vec<u64> {
        let len = self.masks.len();
        for i in 0..len {
            if self.masks[i] {
                self.exclude(i as u64 + self.start);
            }
        }
        self.sift()
    }

    fn reuse(&mut self, new_start: u64) {
        for i in 0..self.masks.len() {
            self.masks[i] = true;
        }
        self.start = new_start;
    }

    fn roll_over(&mut self) {
        self.reuse(self.start + self.masks.len() as u64)
    }
}

pub struct PrimeIt {
    nxt: usize,
    scratch: Sieve,
    primes: Vec<u64>,
}

impl PrimeIt {
    pub fn new() -> Self {
        let mut scratch = Sieve::new(2, 100_000);
        let primes = scratch.exclude_sift();
        PrimeIt {
            nxt: 0,
            scratch,
            primes,
        }
    }
}

impl Iterator for PrimeIt {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.nxt >= self.primes.len() {
            self.scratch.roll_over();
            for p in &self.primes {
                self.scratch.exclude(*p);
            }
            self.primes.append(&mut self.scratch.exclude_sift());
        }
        self.nxt += 1;
        Some(self.primes[self.nxt - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve() {
        let mut s = Sieve::new(0, 10);
        assert_eq!(s.sift(), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        s.exclude(2);
        assert_eq!(s.sift(), vec![0, 1, 2, 3, 5, 7, 9]);

        let mut s = Sieve::new(2, 10);
        assert_eq!(s.sift(), vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        s.exclude(3);
        s.exclude(5);
        assert_eq!(s.sift(), vec![2, 3, 4, 5, 7, 8, 11]);

        assert_eq!(
            Sieve::new(2, 20).exclude_sift(),
            vec![2, 3, 5, 7, 11, 13, 17, 19]
        );
    }
}
