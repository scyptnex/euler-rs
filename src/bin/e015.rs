fn main() {
    println!("{}", solve(20));
}

type Tab = Vec<Vec<Option<u64>>>;

fn solve(side_len: usize) -> u64 {
    let mut tab: Tab = vec![vec![None; side_len + 1]; side_len + 1];
    calc(side_len, side_len, &mut tab)
}

fn calc(x: usize, y: usize, tab: &mut Tab) -> u64 {
    if x == 0 || y == 0 {
        return 1;
    }
    if let Some(c) = tab[x][y] {
        return c;
    }
    let v = calc(x - 1, y, tab) + calc(x, y - 1, tab);
    tab[x][y] = Some(v);
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(2), 6);
    }
}
