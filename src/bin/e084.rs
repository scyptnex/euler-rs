use std::{array::from_fn, collections::HashMap};

use itertools::Itertools;
use num::{BigInt, BigRational, FromPrimitive, One, Zero};

type Distrib = [BigRational; 40];

fn rat(n: u64, d: u64) -> BigRational {
    BigRational::new(BigInt::from_u64(n).unwrap(), BigInt::from_u64(d).unwrap())
}

#[derive(Debug)]
struct Board {
    tiles: Vec<&'static str>,
    idxs: HashMap<&'static str, usize>,
}

impl Board {
    fn new() -> Self {
        let tiles = "go a1 cc1 a2 t1 r1 b1 ch1 b2 b3 
                     jail c1 u1 c2 c3 r2 d1 cc2 d2 d3
                     fp e1 ch2 e2 e3 r3 f1 f2 u2 f3
                     g2j g1 g2 cc3 g3 r4 ch3 h1 t2 h2"
            .split_whitespace()
            .collect_vec();
        assert!(tiles.len() == 40);
        let idxs = tiles.iter().enumerate().map(|(i, s)| (*s, i)).collect();
        Self { tiles, idxs }
    }

    fn roll(&self, dice: usize, pos: usize) -> Distrib {
        let go = 0;
        let jail = 10;
        let g2j = 30;

        let mut ret = from_fn(|_| BigRational::zero());
        let mut doubles = Vec::new();
        for d1 in 1..=dice {
            for d2 in 1..=dice {
                let new_pos = (pos + d1 + d2) % 40;
                ret[new_pos] += rat(1, dice.pow(2) as u64);
                if d1 == d2 {
                    doubles.push(new_pos);
                }
            }
        }

        // 3 doubles in a row goes to jail
        let jc = rat(1, dice.pow(3) as u64);
        for i in 0..40 {
            ret[jail] += &ret[i] * &jc;
            ret[i] = &ret[i] * (BigRational::one() - &jc);
        }

        // Community and chance
        for i in 0..40 {
            if self.tiles[i].starts_with("ch") {
                let c1 = self.idxs["c1"];
                let e3 = self.idxs["e3"];
                let h2 = self.idxs["h2"];
                let r1 = self.idxs["r1"];
                let next_r = (1..)
                    .map(|m| (m + i) % 40)
                    .find(|p| self.tiles[*p].starts_with("r"))
                    .unwrap();
                let next_u = (1..)
                    .map(|m| (m + i) % 40)
                    .find(|p| self.tiles[*p].starts_with("u"))
                    .unwrap();
                let back_3 = (i + 37) % 40;
                ret[jail] += rat(1, 16) * &ret[i];
                ret[go] += rat(1, 16) * &ret[i];
                ret[c1] += rat(1, 16) * &ret[i];
                ret[e3] += rat(1, 16) * &ret[i];
                ret[h2] += rat(1, 16) * &ret[i];
                ret[r1] += rat(1, 16) * &ret[i];
                ret[next_r] += rat(2, 16) * &ret[i];
                ret[next_u] += rat(1, 16) * &ret[i];
                ret[back_3] += rat(1, 16) * &ret[i];
                ret[i] = rat(6, 16) * &ret[i];
            }
        }
        for i in 0..40 {
            if self.tiles[i].starts_with("cc") {
                ret[jail] += rat(1, 16) * &ret[i];
                ret[go] += rat(1, 16) * &ret[i];
                ret[i] = rat(14, 16) * &ret[i];
            }
        }
        // go to jail
        ret[jail] = &ret[jail] + &ret[g2j];
        ret[g2j] = BigRational::zero();

        ret
    }
}

fn solve(dice: usize, iters: usize) -> String {
    let mut dis: Distrib = from_fn(|i| {
        if i == 0 {
            BigRational::one()
        } else {
            BigRational::zero()
        }
    });
    let b = Board::new();
    for _ in 0..iters {
        let mut nd = from_fn(|_| BigRational::zero());
        for p in 0..40 {
            let pd = b.roll(dice, p);
            for pp in 0..40 {
                nd[pp] += &pd[pp] * &dis[p];
            }
        }
        dis = nd;
    }
    let mut dv = dis
        .iter()
        .enumerate()
        .map(|(i, r)| (r, format!("{:02}", i)))
        .collect_vec();
    dv.sort();
    (0..3).map(|i| dv[39 - i].1.clone()).collect()
}

fn main() {
    println!("{}", solve(4, 35));
}
