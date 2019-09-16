impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let half = nums.len() / 2;
        let major_l = Self::majority_element(nums[..half].to_vec());
        let major_r = Self::majority_element(nums[half..].to_vec());

        if major_l == major_r {
            return major_l;
        }

        let mut cnt_l = 0;
        let mut cnt_r = 0;
        for &n in &nums {
            if n == major_l {
                cnt_l += 1;
                if cnt_l > half {
                    return major_l;
                }
            }
            if n == major_r {
                cnt_r += 1;
                if cnt_r > half {
                    return major_r;
                }
            }
        }
        0
    }
}
