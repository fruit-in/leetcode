impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut last_ban = -1;
        let mut last_min = -1;
        let mut last_max = -1;
        let mut ret = 0;

        for i in 0..nums.len() {
            if nums[i] < min_k || nums[i] > max_k {
                last_ban = i as i64;
            }
            if nums[i] == min_k {
                last_min = i as i64;
            }
            if nums[i] == max_k {
                last_max = i as i64;
            }

            if last_min > last_ban && last_max > last_ban {
                ret += last_min.min(last_max) - last_ban;
            }
        }

        ret
    }
}
