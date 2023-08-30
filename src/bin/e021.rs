use euler_rs::factors;

fn main() {
    println!("{}", solve());
}

fn solve() -> u64 {
    (2..10000)
        .filter(is_amicable)
        .inspect(|x| {
            dbg!(x);
        })
        .sum()
}

fn is_amicable(n: &u64) -> bool {
    let m = sod(*n);
    m != *n && sod(m) == *n
}

fn sod(n: u64) -> u64 {
    factors::get_factors(n).filter(|x| *x != n).sum()
}
