use std::fs::read_to_string;

use euler_rs::res::get_input;

fn solve() -> u64 {
    let data: String = read_to_string(get_input("0067_triangle.txt")).unwrap();
    let grid: Vec<Vec<u64>> = data
        .lines()
        .map(|l| l.split(' ').map(|v| v.parse::<u64>().unwrap()).collect())
        .rev()
        .collect();
    let mut best: Vec<u64> = grid.first().unwrap().clone();
    for row in 1..grid.len() {
        let new_best = (0..best.len() - 1)
            .map(|col| grid[row][col] + std::cmp::max(best[col], best[col + 1]))
            .collect();
        best = new_best;
    }
    best[0]
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 7273);
    }
}
