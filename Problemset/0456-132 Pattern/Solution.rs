impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut stack = vec![];
        let mut min_num = i32::MAX;

        for k in 0..nums.len() {
            while let Some(&(numsi, numsj)) = stack.last() {
                if numsi < nums[k] && nums[k] < numsj {
                    return true;
                } else if nums[k] >= numsj {
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push((min_num, nums[k]));
            min_num = min_num.min(nums[k]);
        }

        false
    }
}
