use std::collections::HashSet;

impl Solution {
    pub fn count_excellent_pairs(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut count = vec![0; 32.min(k + 1)];
        let mut ret = 0;

        for num in nums.iter().collect::<HashSet<_>>().into_iter() {
            count[k.min(num.count_ones() as usize)] += 1;
        }

        for i in 1..count.len() {
            for j in k - i..count.len() {
                ret += count[i] * count[j];
            }
        }

        ret
    }
}
