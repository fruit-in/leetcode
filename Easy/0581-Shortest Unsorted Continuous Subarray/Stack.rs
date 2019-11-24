impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let mut l = nums.len();
        let mut r = 0;

        for i in 0..nums.len() {
            if !stack.is_empty() && *stack.last().unwrap() > nums[i] {
                while !stack.is_empty() && *stack.last().unwrap() > nums[i] {
                    stack.pop();
                }
                l = l.min(stack.len());
            }
            stack.push(nums[i]);
        }

        stack.clear();

        for i in (0..nums.len()).rev() {
            if !stack.is_empty() && *stack.last().unwrap() < nums[i] {
                while !stack.is_empty() && *stack.last().unwrap() < nums[i] {
                    stack.pop();
                }
                r = r.max(nums.len() - stack.len() - 1);
            }
            stack.push(nums[i]);
        }

        (r as i32 - l as i32 + 1).max(0)
    }
}
