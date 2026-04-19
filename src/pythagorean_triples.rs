use num::integer::gcd;

pub type Triple = (u64, u64, u64);

#[derive(Debug, Clone, Copy)]
pub struct PrimitiveTriple {
    m: u64,
    n: u64,
}

impl PrimitiveTriple {
    fn new(m: u64, n: u64) -> Self {
        Self { m, n }
    }

    pub fn m(&self) -> u64 {
        self.m
    }

    pub fn n(&self) -> u64 {
        self.n
    }

    pub fn triple(&self) -> Triple {
        let m = self.m;
        let n = self.n;
        (m * m - n * n, 2 * m * n, m * m + n * n)
    }
}

pub struct PrimTriples {
    m: u64,
    n: u64,
}

impl PrimTriples {
    fn new() -> Self {
        PrimTriples { m: 2, n: 1 }
    }
}

impl Iterator for PrimTriples {
    type Item = PrimitiveTriple;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = PrimitiveTriple::new(self.m, self.n);
        loop {
            self.n += 1;
            if self.n == self.m {
                self.m += 1;
                self.n = 1;
                break;
            }
            if self.n % 2 == 1 && self.m % 2 == 1 {
                continue;
            }
            if gcd(self.n, self.m) != 1 {
                continue;
            }
            break;
        }
        Some(ret)
    }
}

pub fn triples() -> PrimTriples {
    PrimTriples::new()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_primitive_triple() {
        let pt = PrimitiveTriple::new(3, 2);
        assert_eq!(pt.m(), 3);
        assert_eq!(pt.n(), 2);
        assert_eq!(pt.triple(), (5, 12, 13));
    }

    #[test]
    fn test_triples() {
        assert_eq!(
            triples().take(5).map(|t| (t.n(), t.m())).collect_vec(),
            vec![(1, 2), (1, 3), (2, 3), (1, 4), (3, 4)]
        );
    }
}
