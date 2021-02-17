impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut even_sums = vec![nums[0]];
        let mut odd_sums = vec![0];
        let mut ret = 0;

        for i in 1..n {
            if i % 2 == 0 {
                even_sums.push(nums[i] + even_sums[i - 1]);
                odd_sums.push(odd_sums[i - 1]);
            } else {
                even_sums.push(even_sums[i - 1]);
                odd_sums.push(nums[i] + odd_sums[i - 1]);
            }
        }

        for i in 0..n {
            let even_sum = match i % 2 {
                0 => even_sums[i] - nums[i] + odd_sums[n - 1] - odd_sums[i],
                _ => even_sums[i] + odd_sums[n - 1] - odd_sums[i],
            };
            let odd_sum = match i % 2 {
                0 => odd_sums[i] + even_sums[n - 1] - even_sums[i],
                _ => odd_sums[i] - nums[i] + even_sums[n - 1] - even_sums[i],
            };

            ret += (even_sum == odd_sum) as i32;
        }

        ret
    }
}
