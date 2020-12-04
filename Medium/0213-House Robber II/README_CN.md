# 213. 打家劫舍 II
你是一个专业的小偷，计划偷窃沿街的房屋，每间房内都藏有一定的现金。这个地方所有的房屋都 **围成一圈** ，这意味着第一个房屋和最后一个房屋是紧挨着的。同时，相邻的房屋装有相互连通的防盗系统，**如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警** 。

给定一个代表每个房屋存放金额的非负整数数组，计算你 **在不触动警报装置的情况下** ，能够偷窃到的最高金额。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,3,2]
<strong>输出:</strong> 3
<strong>解释:</strong> 你不能先偷窃 1 号房屋（金额 = 2），然后偷窃 3 号房屋（金额 = 2）, 因为他们是相邻的。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,1]
<strong>输出:</strong> 4
<strong>解释:</strong> 你可以先偷窃 1 号房屋（金额 = 1），然后偷窃 3 号房屋（金额 = 3）。
     偷窃到的最高金额 = 1 + 3 = 4 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [0]
<strong>输出:</strong> 0
</pre>

#### 提示:
* `1 <= nums.length <= 100`
* `0 <= nums[i] <= 1000`

## 题解 (Rust)

### 1. 动态规划
```Rust
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut rob_fst = (0, 0, nums[0]);
        let mut rob_lst = (0, 0, 0);

        for i in 1..nums.len() {
            if i != nums.len() - 1 {
                rob_fst = (rob_fst.1, rob_fst.2, nums[i] + rob_fst.0.max(rob_fst.1));
            }
            rob_lst = (rob_lst.1, rob_lst.2, nums[i] + rob_lst.0.max(rob_lst.1));
        }

        rob_fst.1.max(rob_fst.2).max(rob_lst.1).max(rob_lst.2)
    }
}
```
