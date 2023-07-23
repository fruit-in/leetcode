impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut result = vec![0];
        for i in 1..=num as usize{
            result.push(&result[i & (i - 1)] + 1);
        }
        result
    }
}
