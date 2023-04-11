impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        match (nums1.len() % 2, nums2.len() % 2) {
            (0, 0) => 0,
            (0, 1) => nums1.iter().fold(0, |acc, x| acc ^ x),
            (1, 0) => nums2.iter().fold(0, |acc, x| acc ^ x),
            _ => nums1.iter().chain(nums2.iter()).fold(0, |acc, x| acc ^ x),
        }
    }
}
