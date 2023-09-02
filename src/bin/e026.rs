use std::collections::HashMap;

fn main() {
    println!("{}", solve(1000));
}

fn solve(lim: u64) -> u64 {
    (2..lim)
        .map(|x| (x, ldrc(x)))
        .max_by_key(|(_, r)| *r)
        .unwrap()
        .0
}

fn ldrc(numer: u64) -> u64 {
    let mut seen = HashMap::<u64, u64>::new();
    let mut rem = 1u64;
    for place in 1.. {
        rem = (10 * rem) % numer;
        if rem == 0 {
            return 0;
        }
        if let Some(sp) = seen.get(&rem) {
            return place - sp;
        }
        seen.insert(rem, place);
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(ldrc(2), 0);
        assert_eq!(ldrc(3), 1);
        assert_eq!(ldrc(7), 6);
        assert_eq!(solve(11), 7);
    }
}
