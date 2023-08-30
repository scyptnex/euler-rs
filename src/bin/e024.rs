fn main() {
    println!("{}", solve(9, 1000000));
}

fn solve(lim: u8, count: usize) -> u64 {
    let nums: Vec<char> = (0..=lim).map(|i| b'0' + i).map(|c| c as char).collect();
    let mut p: Vec<String> = Vec::new();
    perms(nums, Vec::new(), &mut p);
    p.sort();
    p[count - 1].parse().unwrap()
}

fn perms(inp: Vec<char>, cur: Vec<char>, out: &mut Vec<String>) {
    if inp.len() == 0 {
        out.push(cur.iter().collect());
    }
    for c in &inp {
        let mut next = cur.clone();
        next.push(*c);
        perms(
            inp.clone().into_iter().filter(|nc| *nc != *c).collect(),
            next,
            out,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(2, 3), 102);
        assert_eq!(solve(2, 6), 210);
    }
}
