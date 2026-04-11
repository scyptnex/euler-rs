use euler_rs::infile;
use itertools::Itertools;
use petgraph::{algo::dijkstra, Graph};

fn solve() -> u64 {
    let mut g = Graph::<(), u64>::new();
    let node_weights = infile!()
        .lines()
        .map(|ln| {
            ln.split(",")
                .map(|s| (g.add_node(()), s.parse::<u64>().unwrap()))
                .collect_vec()
        })
        .collect_vec();
    let rows = node_weights.len();
    let cols = node_weights.first().unwrap().len();
    for r in 0..rows {
        for c in 0..cols {
            let cur = node_weights[r][c];
            if r > 0 {
                let above = node_weights[r - 1][c];
                g.add_edge(above.0, cur.0, above.1);
            }
            if c > 0 {
                let left = node_weights[r][c - 1];
                g.add_edge(left.0, cur.0, left.1);
            }
            if r < rows - 1 {
                let below = node_weights[r + 1][c];
                g.add_edge(below.0, cur.0, below.1);
            }
            if c < cols - 1 {
                let right = node_weights[r][c + 1];
                g.add_edge(right.0, cur.0, right.1);
            }
        }
    }
    let start = node_weights[0][0].0;
    let pre_end = node_weights.last().unwrap().last().unwrap();
    let end = g.add_node(());
    g.add_edge(pre_end.0, end, pre_end.1);
    let ans = dijkstra(&g, start, Some(end), |e| *e.weight());
    *ans.get(&end).unwrap()
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 425185);
    }
}
