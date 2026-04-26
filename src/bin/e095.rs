use std::{cmp::min, collections::HashMap};

fn make_divs(lim: u64) -> Vec<u64> {
    let mut ret = vec![1; (lim + 1) as usize];
    ret[0] = 0;
    ret[1] = 0; // Excludes itself?
    for i in 2u64.. {
        if i * 2 > lim {
            break;
        }
        for m in 2u64.. {
            if i * m > lim {
                break;
            }
            let idx = m * i;
            ret[idx as usize] += i;
        }
    }
    ret
}

fn solve() -> u64 {
    let cosum = make_divs(1_000_000);
    let mut seen = vec![false; cosum.len()];
    let mut longest = 0;
    let mut longest_small_elem = 0;
    for i in 2..=1_000_000 {
        if seen[i] {
            continue;
        }
        let mut path = HashMap::new();
        let mut cur = i as u64;
        loop {
            if path.contains_key(&cur) {
                let mut cycle_cur = cur;
                let mut cycle_len = 1;
                let mut cycle_min = cur;
                loop {
                    if path[&cycle_cur] == cur {
                        break;
                    }
                    cycle_cur = path[&cycle_cur];
                    cycle_len += 1;
                    cycle_min = min(cycle_min, cycle_cur);
                }
                if cycle_len > longest {
                    longest = cycle_len;
                    longest_small_elem = cycle_min;
                }
            }
            if cur > 1_000_000 {
                break;
            }
            if seen[cur as usize] {
                break;
            }
            seen[cur as usize] = true;
            let nxt = cosum[cur as usize];
            path.insert(cur, nxt);
            cur = nxt;
        }
    }
    longest_small_elem
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let sopd = make_divs(285);
        assert_eq!(sopd[28], 28);
        assert_eq!(sopd[220], 284);
        assert_eq!(sopd[284], 220);
        assert_eq!(solve(), 14316);
    }
}
