impl Solution {
    pub fn max_value_after_reverse(nums: Vec<i32>) -> i32 {
        let origin = (1..nums.len())
            .map(|i| (nums[i] - nums[i - 1]).abs())
            .sum::<i32>();
        let mut max_num = nums[0].min(nums[1]);
        let mut min_num = nums[0].max(nums[1]);
        let mut ret = origin;

        for i in 1..nums.len() - 1 {
            let a = nums[i - 1];
            let b = nums[i];
            let c = nums[i + 1];

            ret = ret.max(origin + (c - nums[0]).abs() - (c - b).abs());
            ret = ret.max(origin + (a - nums.last().unwrap()).abs() - (b - a).abs());

            let x = b.max(c);
            let y = b.min(c);

            if x < max_num {
                ret = ret.max(origin + 2 * max_num - 2 * x);
            }
            if y > min_num {
                ret = ret.max(origin + 2 * y - 2 * min_num);
            }

            max_num = max_num.max(y);
            min_num = min_num.min(x);
        }

        ret
    }
}
