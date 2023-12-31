use num::integer;

/// Return the proper factors including 1, excluding n
///
/// Example:
/// ```
/// use euler_rs::factors::get_factors;
/// let mut factors = get_factors(6).collect::<Vec<_>>();
/// factors.sort();
/// assert_eq!(factors, vec![1, 2, 3]);
/// ```
pub fn get_factors(n: u64) -> impl Iterator<Item = u64> {
    (1..=integer::sqrt(n))
        .filter(move |f| n % f == 0)
        .flat_map(move |x| {
            if x * x == n {
                [x, n].into_iter()
            } else {
                [x, n / x].into_iter()
            }
        })
        .filter(move |x| *x != n)
}
