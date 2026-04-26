fn is_almost_equil(side: u64, base: u64) -> bool {
    let hb = base / 2;
    let sqh = side * side - hb * hb;
    let h = sqh.isqrt();
    // we will only have an integer area if the height is an integer.
    h * h == sqh
}

fn solve() -> u64 {
    let mut ret = 0;
    for i in 1.. {
        // Only odd sided, base has to be even.
        let side = 2 * i + 1;
        if is_almost_equil(side, side + 1) {
            ret += 3 * side + 1;
        }
        if is_almost_equil(side, side - 1) {
            ret += 3 * side - 1;
        }
        if 3 * side > 1_000_000_000 {
            break;
        }
    }
    ret
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert!(is_almost_equil(5, 6));
        assert!(!is_almost_equil(5, 4));
        //assert_eq!(solve(), 518408346);
    }
}
