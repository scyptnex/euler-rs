use euler_rs::decimal::Digs;
use euler_rs::prime::PrimeBank;

fn main() {
    println!("{}", solve());
}

fn solve() -> String {
    let mut pb = PrimeBank::new();
    for s in 1000..=9999 {
        for d in 1000..=9999 {
            if s + d + d >= 10000 {
                continue;
            }
            if s == 1487 && d == 3330 {
                continue;
            }
            if let Some(v) = as_seq(&mut pb, s, d) {
                return format!("{}{}{}", v[0], v[1], v[2]);
            }
        }
    }
    panic!();
}

fn as_seq(pb: &mut PrimeBank, start: u64, diff: u64) -> Option<[u64; 3]> {
    let sd = start + diff;
    let sdd = start + diff + diff;
    if !pb.query(start) || !pb.query(sd) || !pb.query(sdd) {
        return None;
    }
    let start_digs = start.sdigs();
    let sd_digs = sd.sdigs();
    let sdd_digs = sdd.sdigs();
    if start_digs == sd_digs && start_digs == sdd_digs {
        return Some([start, sd, sdd]);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let mut pb = PrimeBank::new();
        assert_eq!(as_seq(&mut pb, 1487, 3330), Some([1487, 4817, 8147]));
    }
}
