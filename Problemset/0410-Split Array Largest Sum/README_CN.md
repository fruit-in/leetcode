# 410. 分割数组的最大值
给定一个非负整数数组 `nums` 和一个整数 `m` ，你需要将这个数组分成 `m` 个非空的连续子数组。

设计一个算法使得这 `m` 个子数组各自和的最大值最小。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [7,2,5,10,8], k = 2
<strong>输出:</strong> 18
<strong>解释:</strong>
一共有四种方法将 nums 分割为 2 个子数组。
其中最好的方式是将其分为 [7,2,5] 和 [10,8] 。
因为此时这两个子数组各自的和的最大值为18，在所有情况中最小。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,4,5], k = 2
<strong>输出:</strong> 9
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,4,4], m = 3
<strong>输出:</strong> 4
</pre>

#### 提示:
* `1 <= nums.length <= 1000`
* <code>0 <= nums[i] <= 10<sup>6</sup></code>
* `1 <= k <= min(50, nums.length)`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![vec![0; k as usize + 1]; nums.len() + 1];

        for i in 1..=nums.len() {
            dp[i][1] = dp[i - 1][1] + nums[i - 1];
        }

        for i in 2..=nums.len() {
            for j in 2..=i.min(k as usize) {
                dp[i][j] = i32::MAX;
                for x in j - 1..i {
                    dp[i][j] = dp[i][j].min(dp[x][j - 1].max(dp[i][1] - dp[x][1]));
                }
            }
        }

        dp[nums.len()][k as usize]
    }
}
```
