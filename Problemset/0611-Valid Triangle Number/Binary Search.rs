impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        let mut ret = 0;

        nums.sort_unstable();

        for i in 0..nums.len() {
            if nums[i] > 0 {
                for j in (i + 1)..nums.len() {
                    match nums[(j + 1)..].binary_search(&(nums[i] + nums[j] - 1)) {
                        Ok(k) => ret += k + 1,
                        Err(k) => ret += k,
                    }
                }
            }
        }

        ret as i32
    }
}
