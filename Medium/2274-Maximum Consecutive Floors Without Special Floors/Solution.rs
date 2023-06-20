impl Solution {
    pub fn max_consecutive(bottom: i32, top: i32, special: Vec<i32>) -> i32 {
        let mut special = special;

        special.push(bottom - 1);
        special.push(top + 1);
        special.sort_unstable();

        (1..special.len())
            .map(|i| special[i] - special[i - 1] - 1)
            .max()
            .unwrap()
    }
}
