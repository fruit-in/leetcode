impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        let mut desc_stack = vec![];
        let mut countl = vec![0; nums.len()];
        let mut countr = vec![0; nums.len()];

        for i in 0..nums.len() {
            while !desc_stack.is_empty() && nums[*desc_stack.last().unwrap() as usize] < nums[i] {
                desc_stack.pop();
            }

            countl[i] = i as i32 - *desc_stack.last().unwrap_or(&-1) - 1;
            desc_stack.push(i as i32);
        }
        desc_stack.clear();
        for i in (0..nums.len()).rev() {
            while !desc_stack.is_empty() && nums[*desc_stack.last().unwrap() as usize] <= nums[i] {
                desc_stack.pop();
            }

            countr[i] = *desc_stack.last().unwrap_or(&(nums.len() as i32)) - i as i32 - 1;
            desc_stack.push(i as i32);
        }

        (0..nums.len())
            .filter(|&i| nums[i] >= left && nums[i] <= right)
            .map(|i| (countl[i] + 1) * (countr[i] + 1))
            .sum()
    }
}
