impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![Vec::new()];
        }

        let mut ret = Vec::new();

        for i in 0..nums.len() {
            let mut nums_clone = nums.clone();
            nums_clone.remove(i);

            let mut back_ret = Self::permute(nums_clone);

            for arr in &mut back_ret {
                arr.push(nums[i]);
            }
            ret.append(&mut back_ret);
        }

        ret
    }
}
