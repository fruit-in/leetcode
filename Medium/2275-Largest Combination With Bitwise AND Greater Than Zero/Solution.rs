impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut count = [0; 24];

        for &x in &candidates {
            for i in 0..24 {
                if x & (1 << i) != 0 {
                    count[i] += 1;
                }
            }
        }

        *count.iter().max().unwrap()
    }
}
