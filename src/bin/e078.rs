use num::{BigInt, One, Zero};

fn solve() -> u64 {
    let mut pn = vec![BigInt::one()]; // p(0) == 1
    loop {
        let n = pn.len();
        pn.push(
            (1..)
                .map(|k| {
                    (
                        (-1isize).pow(k + 1),
                        k * (3 * k - 1) / 2,
                        k * (3 * k + 1) / 2,
                    )
                })
                .take_while(|(_, sub, _)| *sub as usize <= n)
                // .inspect(|v| {
                //     print!("{v:?} ");
                // })
                .map(|(sign, sub1, sub2)| {
                    (
                        sign,
                        pn[n - (sub1 as usize)].clone(),
                        if sub2 as usize > n {
                            BigInt::zero()
                        } else {
                            pn[n - (sub2 as usize)].clone()
                        },
                    )
                })
                .map(|(sign, sub1, sub2)| sign * (sub1 + sub2))
                .sum(),
        );
        if pn.last().unwrap() % 1_000_000 == BigInt::zero() {
            return n as u64;
        }
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
        assert_eq!(solve(), 55374);
    }
}
