use std::collections::HashSet;

impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut nums_set = HashSet::from([nums[0], nums[1]]);
        let mut ret = 0;

        for &num in &nums[2..] {
            nums_set.insert(num);

            if nums_set.contains(&(num - diff)) && nums_set.contains(&(num - 2 * diff)) {
                ret += 1;
            }
        }

        ret
    }
}
