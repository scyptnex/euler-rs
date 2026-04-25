use std::collections::HashSet;

fn solve() -> u64 {
    let mut ones = HashSet::new();
    let mut ens = HashSet::new();
    ones.insert(1);
    ens.insert(89);
    for i in 1..=10_000_000 {
        let mut s = Vec::new();
        let mut cur = i;
        loop {
            s.push(cur);
            let mut nxt = 0;
            let mut cc = cur;
            while cc > 0 {
                let us = cc % 10;
                cc /= 10;
                nxt += us * us;
            }
            if ones.contains(&nxt) {
                ones.extend(s);
                break;
            } else if ens.contains(&nxt) {
                ens.extend(s);
                break;
            }
            cur = nxt;
        }
    }
    ens.len() as u64
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 8581146);
    }
}
