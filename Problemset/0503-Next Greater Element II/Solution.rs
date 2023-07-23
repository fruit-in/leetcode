impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut ret = vec![-1; nums.len()];

        for i in (0..nums.len()).chain(0..nums.len()) {
            while !stack.is_empty() && nums[i] > nums[*stack.last().unwrap()] {
                ret[stack.pop().unwrap()] = nums[i];
            }

            if ret[i] == -1 {
                stack.push(i);
            }
        }

        ret
    }
}
