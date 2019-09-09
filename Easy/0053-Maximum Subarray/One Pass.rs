impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut max_sum = std::i32::MIN;
        let mut min_sum = 0;

        for n in nums {
            min_sum = min_sum.min(sum);
            sum += n;
            max_sum = max_sum.max(sum - min_sum);
        }
        max_sum
    }
}
