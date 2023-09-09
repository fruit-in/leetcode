impl Solution {
    pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        let mut ret = 0;

        for i in 0..nums.len() / 2 {
            let mut x = nums[i];
            let mut y = nums[nums.len() - 1 - i];

            while y > 0 {
                x *= 10;
                y /= 10;
            }

            ret += (x + nums[nums.len() - 1 - i]) as i64;
        }

        if nums.len() % 2 == 1 {
            ret += nums[nums.len() / 2] as i64;
        }

        ret
    }
}
