impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let diff = s1
            .chars()
            .zip(s2.chars())
            .filter(|(c0, c1)| c0 != c1)
            .take(3)
            .collect::<Vec<_>>();

        diff.is_empty() || (diff.len() == 2 && diff[0].0 == diff[1].1 && diff[0].1 == diff[1].0)
    }
}
