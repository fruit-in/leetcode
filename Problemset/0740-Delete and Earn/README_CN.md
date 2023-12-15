# 740. 删除并获得点数
给你一个整数数组 `nums` ，你可以对它进行一些操作。

每次操作中，选择任意一个 `nums[i]` ，删除它并获得 `nums[i]` 的点数。之后，你必须删除 **所有** 等于 `nums[i] - 1` 和 `nums[i] + 1` 的元素。

开始你拥有 `0` 个点数。返回你能通过这些操作获得的最大点数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,4,2]
<strong>输出:</strong> 6
<strong>解释:</strong>
删除 4 获得 4 个点数，因此 3 也被删除。
之后，删除 2 获得 2 个点数。总共获得 6 个点数。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,2,3,3,3,4]
<strong>输出:</strong> 9
<strong>解释:</strong>
删除 3 获得 3 个点数，接着要删除两个 2 和 4 。
之后，再次删除 3 获得 3 个点数，再次删除 3 获得 3 个点数。
总共获得 9 个点数。
</pre>

#### 提示:
* <code>1 <= nums.length <= 2 * 10<sup>4</sup></code>
* <code>1 <= nums[i] <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut sums = vec![0; *nums.iter().max().unwrap() as usize + 1];
        let mut dp = vec![(0, 0); sums.len()];

        for &num in &nums {
            sums[num as usize] += num;
        }

        dp[1] = (0, sums[1]);

        for i in 2..dp.len() {
            dp[i].0 = dp[i - 1].0.max(dp[i - 1].1);
            dp[i].1 = sums[i] + dp[i - 1].0.max(dp[i - 2].1);
        }

        dp[dp.len() - 1].0.max(dp[dp.len() - 1].1)
    }
}
```
