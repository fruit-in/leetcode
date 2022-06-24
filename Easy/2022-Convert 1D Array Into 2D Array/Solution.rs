impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if (m * n) as usize != original.len() {
            return vec![];
        }

        original.chunks(n as usize).map(|a| a.to_vec()).collect()
    }
}
