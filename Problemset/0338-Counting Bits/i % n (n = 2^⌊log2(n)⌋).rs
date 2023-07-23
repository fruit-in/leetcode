impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut n = 1;
        let mut result = vec![0];
        for i in 1..=num as usize{
            n *= (i / n);
            result.push(&result[i % n] + 1);
        }
        result
    }
}
