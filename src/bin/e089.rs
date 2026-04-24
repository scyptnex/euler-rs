use euler_rs::infile;
use itertools::Itertools;

fn from_roman(r: &str) -> u64 {
    let nums = r
        .chars()
        .map(|c| match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!(),
        })
        .collect_vec();
    let mut ret = 0;
    for i in 0..nums.len() {
        ret += nums[i];
        if i > 0 && nums[i - 1] < nums[i] {
            ret -= 2 * nums[i - 1];
        }
    }
    ret
}

fn to_roman(n: u64) -> String {
    let mut n = n;
    let mut ret = Vec::new();
    let digs = vec![
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ];
    while n > 0 {
        for (s, v) in &digs {
            if n >= *v {
                ret.push(s);
                n -= v;
                break;
            }
        }
    }
    ret.iter().join("")
}

fn solve() -> u64 {
    infile!()
        .lines()
        .map(|n| {
            let d = from_roman(n);
            let nn = to_roman(d);
            let saved = n.len() - nn.len();
            saved as u64
        })
        .sum()
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(from_roman("IIIIIIIIIIIIIIII"), 16);
        assert_eq!(from_roman("VIIIIIIIIIII"), 16);
        assert_eq!(from_roman("VVIIIIII"), 16);
        assert_eq!(from_roman("XIIIIII"), 16);
        assert_eq!(from_roman("VVVI"), 16);
        assert_eq!(from_roman("XVI"), 16);
        assert_eq!(from_roman("IXV"), 14);
        assert_eq!(from_roman("XIV"), 14);
        assert_eq!(to_roman(16), String::from("XVI"));
        assert_eq!(to_roman(14), String::from("XIV"));
        assert_eq!(solve(), 743);
    }
}
