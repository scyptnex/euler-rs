use itertools::Itertools;

fn is_perp(a: &(i64, i64), b: &(i64, i64), c: &(i64, i64)) -> bool {
    let bax = a.0 - b.0;
    let bay = a.1 - b.1;
    let bcx = c.0 - b.0;
    let bcy = c.1 - b.1;
    (bax * bcx) + (bay * bcy) == 0
}

fn solve(lim: i64) -> i64 {
    let points = (0..=lim).cartesian_product(0..=lim).skip(1).collect_vec();
    let mut has_right = 0;
    let o = &(0, 0);
    for pi in 0..points.len() {
        for qi in pi + 1..points.len() {
            let p = &points[pi];
            let q = &points[qi];
            if is_perp(p, o, q) || is_perp(q, p, o) || is_perp(p, q, o) {
                has_right += 1;
            }
        }
    }
    has_right
}

fn main() {
    println!("{}", solve(50));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(2), 14);
        assert_eq!(solve(50), 14234);
    }
}
