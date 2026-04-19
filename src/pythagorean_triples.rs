use std::cmp::{max, min};
use std::collections::HashSet;

/// Returns the set of (a, b)s for which a, b, and sqrt(a^2 + b^2)
/// form a primitive pythagorean triple and a <= lim.
pub fn primitive_pythagorean_triples(lim: u64) -> HashSet<(u64, u64)> {
    let mut ret = HashSet::new();
    for n in 1..lim {
        for m_in in 1.. {
            let m = if n % 2 == 0 {
                n + m_in
            } else {
                // n is odd so make m every even number starting at n+1
                (n - 1) + 2 * m_in
            };
            let a = m * m - n * n;
            let b = 2 * m * n;
            let aa = min(a, b);
            let bb = max(a, b);
            if aa > lim {
                break;
            }
            ret.insert((aa, bb));
        }
    }
    ret
}

pub fn pythagorean_triples(lim: u64) -> HashSet<(u64, u64)> {
    primitive_pythagorean_triples(lim)
        .iter()
        .flat_map(|(a, b)| {
            (1..)
                .map(move |k| (k * a, k * b))
                .take_while(|(a, _)| *a <= lim)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive() {
        assert_eq!(
            primitive_pythagorean_triples(10),
            HashSet::from([(3, 4), (5, 12), (7, 24), (8, 15), (9, 40)])
        );
    }
    #[test]
    fn test_all() {
        assert_eq!(
            pythagorean_triples(10),
            HashSet::from([
                (3, 4),
                (6, 8),
                (9, 12),
                (5, 12),
                (10, 24),
                (7, 24),
                (8, 15),
                (9, 40)
            ])
        );
    }
}
