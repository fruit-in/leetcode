impl Solution {
    pub fn array_nesting(mut nums: Vec<i32>) -> i32 {
        let mut ret = 1;

        for i in 0..nums.len() {
            if nums[i] < 0 {
                continue;
            }

            let mut length = 0;
            let mut j = i;
            while nums[j] >= 0 {
                nums[j] = -nums[j] - 1;
                length += 1;
                j = (-nums[j] - 1) as usize;
            }

            ret = ret.max(length);
        }

        ret
    }
}
