impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut ret = vec![];
        nums.sort_unstable();

        if nums.len() % 2 == 1 {
            ret.push(nums.pop().unwrap());
        }

        for i in 0..nums.len() / 2 {
            ret.push(nums[i]);
            ret.push(nums[nums.len() / 2 + i]);
        }

        ret
    }
}
