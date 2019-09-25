# 198. 打家劫舍
你是一个专业的小偷，计划偷窃沿街的房屋。每间房内都藏有一定的现金，影响你偷窃的唯一制约因素就是相邻的房屋装有相互连通的防盗系统，**如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警**。

给定一个代表每个房屋存放金额的非负整数数组，计算你**在不触动警报装置的情况下**，能够偷窃到的最高金额。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,2,3,1]
<strong>输出:</strong> 4
<strong>解释:</strong> 偷窃 1 号房屋 (金额 = 1) ，然后偷窃 3 号房屋 (金额 = 3)。
     偷窃到的最高金额 = 1 + 3 = 4 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [2,7,9,3,1]
<strong>输出:</strong> 12
<strong>解释:</strong> 偷窃 1 号房屋 (金额 = 2), 偷窃 3 号房屋 (金额 = 9)，接着偷窃 5 号房屋 (金额 = 1)。
     偷窃到的最高金额 = 2 + 9 + 1 = 12 。
</pre>

## 题解 (Rust)

### 1. 动态规划
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
