use std::{array::from_fn, collections::HashSet};

use euler_rs::infile;
use itertools::Itertools;

#[derive(Clone, Debug)]
enum SuSquare {
    Known(u8),
    Unknown(HashSet<u8>),
}

impl SuSquare {
    fn new<T>(val: T) -> Self
    where
        u8: From<T>,
    {
        let uv = u8::from(val);
        if uv == 0 {
            Self::Unknown((1..=9).collect())
        } else {
            Self::Known(uv)
        }
    }

    // Returns false if an error is detected
    fn restrict(&mut self, val: u8) -> bool {
        match self {
            SuSquare::Known(v) => *v != val,
            SuSquare::Unknown(possible) => {
                possible.remove(&val);
                true
            }
        }
    }
}

#[derive(Clone)]
struct Sudoku {
    squares: [[SuSquare; 9]; 9],
}

impl Sudoku {
    fn new() -> Self {
        Self {
            squares: from_fn(|_| from_fn(|_| SuSquare::new(0))),
        }
    }

    fn set(&mut self, r: usize, c: usize, v: u8) -> bool {
        self.squares[r][c] = SuSquare::Known(v);
        let br = r / 3;
        let bc = c / 3;
        for i in 0..9 {
            if i != r && !self.squares[i][c].restrict(v) {
                return false;
            }
            if i != c && !self.squares[r][i].restrict(v) {
                return false;
            }
            let rr = 3 * br + i / 3;
            let cc = 3 * bc + i % 3;
            if (rr != r || cc != c) && !self.squares[rr][cc].restrict(v) {
                return false;
            }
        }
        true
    }
}

fn solve_sudoku(su: Sudoku) -> Option<Sudoku> {
    let mut su = su;
    let mut best;
    loop {
        let mut work = Vec::new();
        best = (0, 0, HashSet::new());
        for r in 0..9 {
            for c in 0..9 {
                match &su.squares[r][c] {
                    SuSquare::Known(_) => (),
                    SuSquare::Unknown(possibles) => {
                        if possibles.len() == 0 {
                            return None;
                        }
                        if possibles.len() == 1 {
                            work.push((r, c, *possibles.iter().next().unwrap()));
                        }
                        if best.2.is_empty() || possibles.len() < best.2.len() {
                            best = (r, c, possibles.clone());
                        }
                    }
                }
            }
        }
        if work.len() == 0 {
            break;
        }
        for (r, c, v) in work {
            if !su.set(r, c, v) {
                return None;
            }
        }
    }
    if best.2.is_empty() {
        return Some(su);
    }
    for possible in best.2 {
        let mut susu = su.clone();
        assert!(susu.set(best.0, best.1, possible));
        if let Some(ret) = solve_sudoku(susu) {
            return Some(ret);
        }
    }
    None
}

fn susolve(v: Vec<&str>) -> u64 {
    let mut su = Sudoku::new();
    for r in 0..9 {
        for c in 0..9 {
            let sq = v[r].as_bytes()[c] - b'0';
            if sq == 0 {
                continue;
            }
            su.set(r, c, sq);
        }
    }
    su = solve_sudoku(su).unwrap();
    let v = su.squares[0]
        .iter()
        .take(3)
        .map(|ss| match ss {
            SuSquare::Known(v) => *v as u64,
            _ => panic!(),
        })
        .collect_vec();
    100 * v[0] + 10 * v[1] + v[2]
}

fn solve() -> u64 {
    infile!()
        .lines()
        .chunks(10)
        .into_iter()
        .map(|x| susolve(x.skip(1).collect_vec()))
        .sum()
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    const UNSOLVED: &'static str = "003020600
900305001
001806400
008102900
700000008
006708200
002609500
800203009
005010300";

    #[test]
    fn test_solve() {
        assert_eq!(susolve(UNSOLVED.lines().collect_vec()), 483);
        assert_eq!(solve(), 24702);
    }
}
