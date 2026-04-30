use itertools::Itertools;
use num::{BigInt, BigRational, FromPrimitive, One, Zero};

type Coefs = Vec<BigRational>;

fn get_true_coefs() -> Coefs {
    (0..=10)
        .map(|i| if i % 2 == 0 { 1 } else { -1 })
        .map(|i| BigRational::new(BigInt::from(i), BigInt::one()))
        .collect_vec()
}

fn calc(n: i64, cf: &Coefs) -> BigRational {
    cf.iter()
        .enumerate()
        .map(|(i, c)| BigRational::from_i64(n).unwrap().pow(i as i32) * c)
        .sum()
}

fn determine(terms: &Vec<BigRational>) -> Coefs {
    if terms.len() == 1 {
        return terms.clone();
    }
    let mut answ = terms.clone();
    let mut grps = (0..terms.len())
        .map(|row| {
            (0..terms.len())
                .map(|p| BigRational::from_i64(row as i64 + 1).unwrap().pow(p as i32))
                .collect_vec()
        })
        .collect_vec();
    for e_row in 0..terms.len() - 1 {
        for x_row in e_row + 1..terms.len() {
            // Its x - fac*e = 0, so fac is x/e
            let fac = &grps[x_row][e_row] / &grps[e_row][e_row];
            // subtract from the answer vector
            answ[x_row] = &answ[x_row] - &fac * &answ[e_row];
            // And from the coefficient matrix
            for i in 0..terms.len() {
                grps[x_row][i] = &grps[x_row][i] - &fac * &grps[e_row][i];
            }
        }
    }
    let mut solns = Vec::<BigRational>::new();
    for solv in 0..terms.len() {
        let mut subt = BigRational::zero();
        let solv_row = grps.len() - 1 - solv;
        for sub in 0..solv {
            subt = &subt - &solns[sub] * &grps[solv_row][grps.len() - 1 - sub];
        }
        let sol_ans = &answ[solv_row] + &subt;
        let cur_sol = &sol_ans / &grps[solv_row][solv_row];
        solns.push(cur_sol);
    }
    solns.into_iter().rev().collect_vec()
}

fn sum_of_fit_bop(cf: Coefs) -> BigRational {
    let needed_terms = cf.len() as i64;
    let mut terms = Vec::new();
    let mut sofb = BigRational::zero();
    for n in 1..=needed_terms - 1 {
        terms.push(calc(n, &cf));
        let ccf = determine(&terms);
        sofb += calc(n + 1, &ccf)
    }
    sofb
}

fn solve() -> BigRational {
    sum_of_fit_bop(get_true_coefs())
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::Zero;

    #[test]
    fn test_solve() {
        assert_eq!(
            sum_of_fit_bop(vec![
                BigRational::zero(),
                BigRational::zero(),
                BigRational::zero(),
                BigRational::one()
            ]),
            BigRational::from_i64(74).unwrap()
        );
        assert_eq!(solve(), BigRational::from_i64(37076114526).unwrap());
    }
}
