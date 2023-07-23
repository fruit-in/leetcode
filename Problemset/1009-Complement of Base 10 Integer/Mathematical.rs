impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        match n {
            0 => 1,
            _ => 2_i32.pow((n as f64).log2() as u32 + 1) - 1 - n,
        }
    }
}
