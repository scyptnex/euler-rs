use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

// Returns the set of (a, b)s for which a, b, and sqrt(a^2 + b^2)
// form a primitive pythagorean triple.
fn primitive_pythagorean_triples(lim: u64) -> HashSet<(u64, u64)> {
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

fn solve() -> u64 {
    let lim = 2000;
    let prim_triples = primitive_pythagorean_triples(lim);
    let all_triples: HashSet<(u64, u64)> = prim_triples
        .iter()
        .flat_map(|(a, b)| {
            (1..)
                .map(move |k| (k * a, k * b))
                .take_while(|(a, _)| *a <= lim)
        })
        .collect();
    let mut a_to_bs = HashMap::<u64, HashSet<u64>>::new();
    let mut b_to_as = HashMap::<u64, HashSet<u64>>::new();
    for (a, b) in &all_triples {
        a_to_bs.entry(*a).or_default().insert(*b);
        b_to_as.entry(*b).or_default().insert(*a);
    }
    //panic!();
    let mut total = 0;
    for longest_side in 1.. {
        // option 1: l >= m+s
        if b_to_as.contains_key(&longest_side) {
            for m_p_s in &b_to_as[&longest_side] {
                for s in 1.. {
                    let m = m_p_s - s;
                    if s > m {
                        break;
                    }
                    total += 1;
                }
            }
        }
        // option 2: l < m+s
        if a_to_bs.contains_key(&longest_side) {
            for m_p_s in &a_to_bs[&longest_side] {
                if *m_p_s > 2 * longest_side {
                    continue;
                }
                for s in 1.. {
                    let m = m_p_s - s;
                    if m > longest_side {
                        continue;
                    }
                    if s > m {
                        break;
                    }
                    total += 1;
                }
            }
        }
        if total > 1_000_000 {
            return longest_side;
        }
        if longest_side == 99 {
            assert_eq!(total, 1975);
        }
        if longest_side == 100 {
            assert_eq!(total, 2060);
        }
    }
    unreachable!();
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 1818);
    }
}
