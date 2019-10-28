impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        2_i32.pow((num as f64).log2() as u32 + 1) - 1 - num
    }
}
