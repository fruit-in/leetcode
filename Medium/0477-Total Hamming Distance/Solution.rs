impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut ret = 0;

        for i in 0..30 {
            let mut zeros = 0;
            let mut ones = 0;
            for num in &nums {
                match (1 << i) & num {
                    0 => zeros += 1,
                    _ => ones += 1,
                }
            }
            ret += zeros * ones;
        }

        ret
    }
}
