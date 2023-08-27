fn main() {
    println!("{}", solve());
}

fn solve() -> u64 {
    for a in 1..1000 {
        for b in a + 1..1000 {
            for c in b + 1..1000 {
                if a * a + b * b != c * c {
                    continue;
                }
                if a + b + c != 1000 {
                    continue;
                }
                return a * b * c;
            }
        }
    }
    panic!();
}
