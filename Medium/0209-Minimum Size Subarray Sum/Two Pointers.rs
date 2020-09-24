impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        if nums.iter().sum::<i32>() < s {
            return 0;
        }

        let mut i = 0;
        let mut sum = 0;
        let mut ret = std::i32::MAX;

        for j in 0..nums.len() {
            sum += nums[j];
            while sum >= s {
                ret = ret.min((j - i) as i32 + 1);
                sum -= nums[i];
                i += 1;
            }
        }

        ret
    }
}
