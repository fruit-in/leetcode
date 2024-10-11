impl Solution {
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut lo = *nums.iter().min().unwrap() as i64;
        let mut hi = *nums.iter().max().unwrap() as i64;

        while lo < hi {
            let m = (lo + hi) / 2;
            let mut x = nums[n - 1] as i64;

            for i in (1..n).rev() {
                if x > m {
                    x += nums[i - 1] as i64 - m;
                } else {
                    x = nums[i - 1] as i64;
                }
            }

            if x > m {
                lo = m + 1;
            } else {
                hi = m;
            }
        }

        hi as i32
    }
}
