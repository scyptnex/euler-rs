use euler_rs::infile;

fn solve() -> u64 {
    let s = infile!();
    let lns = s.lines().filter(|s| !s.is_empty()).collect::<Vec<&str>>();
    let mut front = vec![(0u64, vec![0usize; lns.len()])];
    loop {
        let mut new_front = Vec::new();
        for (f_cur, f_track) in front {
            for nxt in 0..10 {
                let fcn = f_cur * 10 + nxt;
                let ftn: Vec<_> = f_track
                    .iter()
                    .enumerate()
                    .map(|(i, ft)| {
                        if *ft >= lns[i].len() {
                            *ft
                        } else if (lns[i].as_bytes()[*ft] - b'0') as u64 == nxt {
                            *ft + 1
                        } else {
                            *ft
                        }
                    })
                    .collect();
                if ftn == f_track {
                    continue;
                }
                if ftn.iter().enumerate().all(|(i, ft)| *ft >= lns[i].len()) {
                    return fcn;
                }
                new_front.push((fcn, ftn));
            }
        }
        front = new_front;
    }
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 73162890);
    }
}
