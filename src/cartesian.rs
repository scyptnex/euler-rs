use std::ops::Range;

pub fn cartesian(a: Range<i64>, b: Range<i64>) -> impl Iterator<Item = (i64, i64)> {
    (a.start..a.end).flat_map(move |ai| (b.start..b.end).map(move |bi| (ai, bi)))
}
