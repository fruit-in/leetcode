impl Solution {
    pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        let mut nums = (0..nums.len())
            .map(|i| (nums[i] as i64, cost[i] as i64))
            .collect::<Vec<_>>();
        let mut prefix_cost = 0;
        let mut suffix_cost = 0;
        let mut equal_cost = 0;
        let mut total_cost = 0;
        let mut i = 0;
        let mut ret = i64::MAX;

        nums.sort_unstable();

        for j in 0..nums.len() {
            suffix_cost += nums[j].1;
            total_cost += nums[j].1 * (nums[j].0 - nums[0].0 + 1);
        }

        ret = ret.min(total_cost);

        for x in nums[0].0..=nums[nums.len() - 1].0 {
            prefix_cost += equal_cost;
            equal_cost = 0;
            total_cost += prefix_cost - suffix_cost;
            ret = ret.min(total_cost);
            while i < nums.len() && nums[i].0 == x {
                suffix_cost -= nums[i].1;
                equal_cost += nums[i].1;
                i += 1;
            }
        }

        ret
    }
}
