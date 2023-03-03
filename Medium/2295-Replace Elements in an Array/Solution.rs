use std::collections::HashMap;

impl Solution {
    pub fn array_change(nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums;
        let mut indices = nums
            .iter()
            .enumerate()
            .map(|(i, &num)| (num, i))
            .collect::<HashMap<_, _>>();

        for operation in operations {
            let i = indices.remove(&operation[0]).unwrap();
            nums[i] = operation[1];
            indices.insert(nums[i], i);
        }

        nums
    }
}
