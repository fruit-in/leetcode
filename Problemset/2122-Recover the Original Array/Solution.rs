use std::collections::HashMap;

impl Solution {
    pub fn recover_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut arr = vec![];

        nums.sort_unstable();

        for i in 1..nums.len() {
            if nums[i] % 2 != nums[0] % 2 || nums[i] == nums[0] {
                continue;
            }

            let k = (nums[i] - nums[0]) / 2;
            let mut count = HashMap::from([(nums[0], 1)]);

            for j in 1..nums.len() {
                if *count.get(&(nums[j] - 2 * k)).unwrap_or(&0) > 0 {
                    *count.get_mut(&(nums[j] - 2 * k)).unwrap() -= 1;
                    arr.push(nums[j] - k);
                } else {
                    *count.entry(nums[j]).or_insert(0) += 1;
                }
            }

            if arr.len() * 2 == nums.len() {
                return arr;
            } else {
                arr.clear();
            }
        }

        unreachable!()
    }
}
