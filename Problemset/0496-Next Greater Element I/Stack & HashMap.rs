use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut num_greater = HashMap::new();

        for i in 0..nums2.len() {
            while !stack.is_empty() && *stack.last().unwrap() < nums2[i] {
                num_greater.insert(stack.pop().unwrap(), nums2[i]);
            }

            stack.push(nums2[i]);
        }

        nums1.iter().map(|x| *num_greater.get(x).unwrap_or(&-1)).collect()
    }
}
