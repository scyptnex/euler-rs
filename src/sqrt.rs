#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct ContinuedFraction {
    a: isize,
    x: usize,
    b: isize,
    c: isize,
}

impl ContinuedFraction {
    pub fn new(x: usize) -> Self {
        Self {
            a: 1,
            x,
            b: 0,
            c: 1,
        }
    }

    fn to_f64(&self) -> f64 {
        let estim = self.x as f64;
        let estim = estim.sqrt() + (self.b as f64);
        let estim = estim * (self.a as f64);
        estim / (self.c as f64)
    }

    fn estimate(&self) -> isize {
        (self.to_f64().floor()) as isize
    }

    fn pull_term(&mut self) -> usize {
        // Pull out the integer part.
        let t = self.estimate();
        let abmtc = self.a * self.b - self.c * t;
        assert!(abmtc % self.a == 0);
        self.b = abmtc / self.a;
        // Now we are <1, so divide.
        let new_a = self.c;
        let new_b = -self.b;
        let new_c = self.a * (self.x as isize) - self.a * self.b * self.b;
        // And simplify.
        assert!(new_c % new_a == 0);
        self.c = new_c / new_a;
        self.a = 1;
        self.b = new_b;
        t as usize
    }
}

impl Iterator for ContinuedFraction {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.pull_term())
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(
            ContinuedFraction::new(23).take(8).collect_vec(),
            vec![4, 1, 3, 1, 8, 1, 3, 1]
        );
    }
}
