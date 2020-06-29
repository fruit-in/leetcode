impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![Vec::new()];
        }

        let mut nums = nums;
        nums.sort_unstable();
        let mut ret = Vec::new();

        for i in 0..nums.len() {
            if i == 0 || nums[i] != nums[i - 1] {
                let mut nums_clone = nums.clone();
                nums_clone.remove(i);

                let mut back_ret = Self::permute_unique(nums_clone);

                for arr in &mut back_ret {
                    arr.push(nums[i]);
                }
                ret.append(&mut back_ret);
            }
        }

        ret
    }
}
