impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut i = 0;
        let mut count = 0;
        let mut ret = n;

        nums.sort_unstable();

        for j in 0..nums.len() {
            if j == 0 || nums[j] != nums[j - 1] {
                count += 1;
            }
            while nums[j] - nums[i] > n - 1 {
                if i == 0 || nums[i] != nums[i - 1] {
                    count -= 1;
                }
                i += 1;
            }

            ret = ret.min(n - count);
        }

        ret
    }
}
