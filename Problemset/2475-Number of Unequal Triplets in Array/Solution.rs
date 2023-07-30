use std::collections::HashMap;

impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let mut ret = 0;

        for num in &nums {
            count.entry(num).and_modify(|c| *c += 1).or_insert(1);
        }

        let count = count.values().collect::<Vec<_>>();

        for i in 0..count.len() {
            for j in i + 1..count.len() {
                for k in j + 1..count.len() {
                    ret += count[i] * count[j] * count[k];
                }
            }
        }

        ret
    }
}
