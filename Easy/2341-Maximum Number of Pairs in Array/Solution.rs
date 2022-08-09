use std::collections::HashMap;

impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut count = HashMap::new();
        let mut answer = vec![0, 0];

        for num in nums {
            *count.entry(num).or_insert(0) += 1;
        }

        answer[0] = count.values().map(|&x| x / 2).sum();
        answer[1] = count.values().map(|&x| x % 2).sum();

        answer
    }
}
