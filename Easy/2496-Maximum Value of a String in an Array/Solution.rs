impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        strs.into_iter()
            .map(|s| s.parse().unwrap_or(s.len() as i32))
            .max()
            .unwrap()
    }
}
