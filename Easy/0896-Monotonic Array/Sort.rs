impl Solution {
    pub fn is_monotonic(a: Vec<i32>) -> bool {
        let mut sort_a = a.clone();
        sort_a.sort_unstable();

        if sort_a == a {
            true
        } else {
            sort_a.reverse();
            sort_a == a
        }
    }
}
