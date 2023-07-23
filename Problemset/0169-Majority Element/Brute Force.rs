impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            let mut cnt = 0;
            for j in i..nums.len() {
                if nums[j] == nums[i] {
                    cnt += 1;
                }
                if cnt > nums.len() / 2 {
                    return nums[i];
                }
            }
        }
        0
    }
}
