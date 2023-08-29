fn main() {
    println!("{}", solve(1000));
}

fn solve(max: u64) -> u64 {
    (1..=max).map(lent).sum()
}

fn lent(n: u64) -> u64 {
    dbg!(letters(n)).len() as u64
}

fn letters(n: u64) -> String {
    match n {
        0 => "".to_owned(),
        1 => "one".to_owned(),
        2 => "two".to_owned(),
        3 => "three".to_owned(),
        4 => "four".to_owned(),
        5 => "five".to_owned(),
        6 => "six".to_owned(),
        7 => "seven".to_owned(),
        8 => "eight".to_owned(),
        9 => "nine".to_owned(),
        10 => "ten".to_owned(),
        11 => "eleven".to_owned(),
        12 => "twelve".to_owned(),
        13 => "thirteen".to_owned(),
        14 => "fourteen".to_owned(),
        15 => "fifteen".to_owned(),
        16 => "sixteen".to_owned(),
        17 => "seventeen".to_owned(),
        18 => "eighteen".to_owned(),
        19 => "nineteen".to_owned(),
        20..=29 => format!("twenty{}", letters(n % 10)),
        30..=39 => format!("thirty{}", letters(n % 10)),
        40..=49 => format!("forty{}", letters(n % 10)),
        50..=59 => format!("fifty{}", letters(n % 10)),
        60..=69 => format!("sixty{}", letters(n % 10)),
        70..=79 => format!("seventy{}", letters(n % 10)),
        80..=89 => format!("eighty{}", letters(n % 10)),
        90..=99 => format!("ninety{}", letters(n % 10)),
        100..=999 => format!(
            "{}hundred{}{}",
            letters(n / 100),
            if n % 100 == 0 { "" } else { "and" },
            letters(n % 100)
        ),
        1000 => "onethousand".to_owned(),
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(letters(300), "threehundred");
        assert_eq!(letters(309), "threehundredandnine");
        assert_eq!(letters(390), "threehundredandninety");
        assert_eq!(letters(399), "threehundredandninetynine");
        assert_eq!(lent(342), 23);
        assert_eq!(lent(115), 20);
        assert_eq!(solve(5), 19);
    }
}
