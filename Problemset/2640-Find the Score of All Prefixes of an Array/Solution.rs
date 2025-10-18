impl Solution {
    pub fn find_prefix_score(nums: Vec<i32>) -> Vec<i64> {
        let n = nums.len();
        let mut max_num = 0;
        let mut conver = vec![0; n];
        let mut ans = vec![0; n];

        for i in 0..n {
            max_num = max_num.max(nums[i]);
            conver[i] = nums[i] + max_num;
            ans[i] = ans[i.max(1) - 1] + conver[i] as i64;
        }

        ans
    }
}
