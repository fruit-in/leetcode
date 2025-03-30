impl Solution {
    pub fn maximize_greatness(mut nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut ret = 0;

        nums.sort_unstable();

        for j in 0..nums.len() {
            while i < nums.len() && nums[i] <= nums[j] {
                i += 1;
            }

            if i >= nums.len() {
                break;
            } else {
                i += 1;
                ret += 1;
            }
        }

        ret
    }
}
