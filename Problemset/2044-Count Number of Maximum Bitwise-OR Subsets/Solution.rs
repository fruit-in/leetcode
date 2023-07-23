use std::collections::HashMap;

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut count0 = HashMap::from([(0, 1)]);
        let mut count1 = HashMap::new();

        for &num in &nums {
            count1.clear();

            for (&x, &y) in count0.iter() {
                count1.entry(x | num).and_modify(|c| *c += y).or_insert(y);
            }

            for (&x, &y) in count1.iter() {
                count0.entry(x).and_modify(|c| *c += y).or_insert(y);
            }
        }

        count0[&nums.iter().fold(0, |a, b| a | b)]
    }
}
