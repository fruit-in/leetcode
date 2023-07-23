use std::collections::HashMap;

impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let mut max_count = 0;
        let mut ret = -1;

        for num in nums.iter().filter(|&&x| x % 2 == 0) {
            count.entry(num).and_modify(|x| *x += 1).or_insert(1);
        }

        for (&k, v) in count {
            if v > max_count || (v == max_count && k < ret) {
                max_count = v;
                ret = k;
            }
        }

        ret
    }
}
