impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut cnt = 1;
        let mut major = nums[0];
        for &n in &nums[1..] {
            if cnt == 0 {
                major = n;
            }
            if n == major {
                cnt += 1;
            } else {
                cnt -= 1;
            }
        }
        major
    }
}
