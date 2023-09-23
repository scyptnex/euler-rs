fn main() {
    println!("{}", solve());
}

fn solve() -> u64 {
    (3..=1000)
        .map(|p| (p, perim_count(p)))
        .max_by_key(|(_, pc)| *pc)
        .unwrap()
        .0
}

fn perim_count(perim: u64) -> usize {
    let mut count = 0;
    for short in 1..perim - 2 {
        for long in short + 1..perim - short {
            let c = perim - long - short;
            if long * long + short * short != c * c {
                continue;
            }
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(perim_count(120), 3);
    }
}
