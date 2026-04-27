use euler_rs::infile;
use num::{BigUint, Num};

fn solvel(inf: &str) -> u64 {
    inf.lines()
        .enumerate()
        .map(|(i, l)| {
            let (b, e) = l.split_once(',').unwrap();
            dbg!(i);
            (
                BigUint::from_str_radix(b, 10)
                    .unwrap()
                    .pow(e.parse().unwrap()),
                i + 1,
            )
        })
        .max()
        .unwrap()
        .1 as u64
}

fn solve() -> u64 {
    solvel(&infile!())
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solvel("519432,525806\n632382,518061"), 2);
        //assert_eq!(solve(), 709);
    }
}
