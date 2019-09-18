# 198. House Robber
You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security system connected and **it will automatically contact the police if two adjacent houses were broken into on the same night**.

Given a list of non-negative integers representing the amount of money of each house, determine the maximum amount of money you can rob tonight **without alerting the police**.

#### Example 1:
<pre>
<strong>Input:</strong> [1,2,3,1]
<strong>Output:</strong> 4
<strong>Explanation:</strong> Rob house 1 (money = 1) and then rob house 3 (money = 3).
             Total amount you can rob = 1 + 3 = 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [2,7,9,3,1]
<strong>Output:</strong> 12
<strong>Explanation:</strong> Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
             Total amount you can rob = 2 + 9 + 1 = 12.
</pre>

## Solutions (Rust)

### 1. Dynamic Programming
```Rust
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return *nums.iter().max().unwrap_or(&0);
        }
        let mut dp = vec![
            nums[0],
            nums[0].max(nums[1]),
            nums[1].max(nums[0] + nums[2])
        ];
        for i in 3..nums.len() {
            dp.push((dp[i - 3] + nums[i - 1]).max(dp[i - 2] + nums[i]));
        }
        dp.pop().unwrap()
    }
}
```
