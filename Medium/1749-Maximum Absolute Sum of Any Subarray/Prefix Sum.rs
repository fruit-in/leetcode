impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut max_sum = 0;
        let mut min_sum = 0;
        let mut ret = 0;

        for x in nums {
            sum += x;
            max_sum = max_sum.max(sum);
            min_sum = min_sum.min(sum);
            ret = ret.max((sum - max_sum).abs()).max((sum - min_sum).abs());
        }

        ret
    }
}
