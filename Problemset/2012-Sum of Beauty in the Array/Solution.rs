impl Solution {
    pub fn sum_of_beauties(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max = vec![i32::MIN; nums.len()];
        let mut min = vec![i32::MAX; nums.len()];
        let mut ret = 0;

        for i in 1..nums.len() {
            max[i] = max[i - 1].max(nums[i - 1]);
            min[n - 1 - i] = min[n - i].min(nums[n - i]);
        }

        for i in 1..nums.len() - 1 {
            if max[i] < nums[i] && nums[i] < min[i] {
                ret += 2;
            } else if nums[i - 1] < nums[i] && nums[i] < nums[i + 1] {
                ret += 1;
            }
        }

        ret
    }
}
