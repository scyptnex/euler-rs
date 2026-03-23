use euler_rs::prime::PrimeSieve;

fn solve() -> usize {
    let pb = PrimeSieve::new(700_000_000);
    let mut sl = 1;
    let mut cur = 1;
    let mut crnrs = 1;
    let mut pms = 0;
    loop {
        sl += 2;
        for _ in 0..4 {
            cur += sl - 1;
            if pb.is_prime(cur) {
                pms += 1;
            }
        }
        crnrs += 4;
        if sl % 1000 == 1 {
            println!("{}, {}", sl, (pms as f64) / (crnrs as f64));
        }
        if 10 * pms < crnrs {
            return sl;
        }
    }
}

fn main() {
    println!("{}", solve());
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_solve() {
//         assert_eq!(solve(), 26241);
//     }
// }
