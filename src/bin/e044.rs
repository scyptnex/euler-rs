fn main() {
    println!("{}", solve());
}

fn solve() -> u64 {
    let pents: Vec<u64> = (1..10_000_000).map(|n| n * (3 * n - 1) / 2).collect();
    let pentss: std::collections::HashSet<_> = pents.iter().collect();
    for pd in pents.iter() {
        for pi in 0.. {
            if pents[pi + 1] - pents[pi] > *pd {
                break;
            }
            let p2 = pents[pi] + pd;
            if pentss.contains(&p2) && pentss.contains(&(pents[pi] + p2)) {
                return *pd;
            }
        }
    }
    panic!();
}
