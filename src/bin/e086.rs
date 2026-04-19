use std::collections::{HashMap, HashSet};

use euler_rs::pythagorean_triples::pythagorean_triples;

fn solve() -> u64 {
    let lim = 2000;
    let all_triples = pythagorean_triples(lim);
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
