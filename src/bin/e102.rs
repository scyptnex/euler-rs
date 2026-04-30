use euler_rs::infile;
use itertools::Itertools;

fn solve() -> usize {
    infile!()
        .lines()
        .map(|l| {
            l.split(',')
                .flat_map(str::parse::<i64>)
                .chunks(2)
                .into_iter()
                .map(|c| {
                    let mut c = c;
                    (c.next().unwrap(), c.next().unwrap())
                })
                .collect_vec()
        })
        .filter(|t| {
            let v = (0..3)
                .map(|i| {
                    let ni = (i + 1) % 3;
                    let co = (-t[i].0, -t[i].1);
                    let cn = (t[ni].0 - t[i].0, t[ni].1 - t[i].1);
                    // cross product
                    (cn.0 * co.1) - (cn.1 * co.0)
                })
                .collect_vec();
            // all the cross products are on the same "side" of the edges.
            v.iter().all(|dr| dr >= &0) || v.iter().all(|dr| dr <= &0)
        })
        .count()
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 228);
    }
}
