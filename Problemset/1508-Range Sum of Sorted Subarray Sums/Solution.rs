impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let mut sums = vec![];
        let mut ret = 0;

        for i in 0..nums.len() {
            let mut sum = 0;
            for j in i..nums.len() {
                sum += nums[j];
                sums.push(sum);
            }
        }

        sums.sort_unstable();

        for i in left..=right {
            ret = (ret + sums[i as usize - 1]) % 1_000_000_007;
        }

        ret
    }
}
