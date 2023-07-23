impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable_by(|a, b| b.cmp(a));

        for x in 1..=nums.len() {
            if nums[x - 1] >= x as i32 && (x == nums.len() || nums[x] < x as i32) {
                return x as i32;
            }
        }

        -1
    }
}
