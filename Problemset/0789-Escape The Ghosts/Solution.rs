impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        ghosts
            .iter()
            .map(|v| (v[0] - target[0]).abs() + (v[1] - target[1]).abs())
            .min()
            .unwrap_or(20001)
            > (target[0].abs() + target[1].abs())
    }
}
