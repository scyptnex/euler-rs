fn main() {
    println!("{}", solve());
}

fn solve() -> u64 {
    (2..1_000_000)
        .map(|i| (i, Clz::new(i).count()))
        .max_by_key(|(_, l)| *l)
        .unwrap()
        .0
}

struct Clz {
    nxt: u64,
    ended: bool,
}

impl Clz {
    fn new(i: u64) -> Self {
        Clz {
            nxt: i,
            ended: false,
        }
    }
}

impl Iterator for Clz {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        if self.ended {
            return None;
        }
        let y = self.nxt;
        self.nxt = match y % 2 {
            0 => y / 2,
            _ => 3 * y + 1,
        };
        if y == 1 {
            self.ended = true;
        }
        Some(y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(Clz::new(13).count(), 10);
    }
}
