impl Solution {
    pub fn find_min_fibonacci_numbers(mut k: i32) -> i32 {
        let mut nums = vec![1, 1];
        let mut i = 1;
        let mut ret = 0;

        while nums[i] < k {
            nums.push(nums[i - 1] + nums[i]);
            i += 1;
        }

        while k > 0 {
            while nums[i] > k {
                i -= 1;
            }
            k -= nums[i];
            ret += 1;
        }

        ret
    }
}
