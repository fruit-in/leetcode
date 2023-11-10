# 1027. 最长等差数列
给你一个整数数组 `nums`，返回 `nums` 中最长等差子序列的**长度**。

回想一下，`nums` 的子序列是一个列表 <code>nums[i<sub>1</sub>], nums[i<sub>2</sub>], ..., nums[i<sub>k</sub>]</code> ，且 <code>0 <= i<sub>1</sub> < i<sub>2</sub> < ... < i<sub>k</sub> <= nums.length - 1</code>。并且如果 `seq[i+1] - seq[i]`( `0 <= i < seq.length - 1`) 的值都相同，那么序列 `seq` 是等差的。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,6,9,12]
<strong>输出:</strong> 4
<strong>解释:</strong>
整个数组是公差为 3 的等差数列。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [9,4,7,2,10]
<strong>输出:</strong> 3
<strong>解释:</strong>
最长的等差子序列是 [4,7,10]。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [20,1,15,3,10,5,8]
<strong>输出:</strong> 4
<strong>解释:</strong>
最长的等差子序列是 [20,15,10,5]。
</pre>

#### 提示:
* `2 <= nums.length <= 1000`
* `0 <= nums[i] <= 500`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let max_num = *nums.iter().max().unwrap();
        let mut dp = vec![vec![0; max_num as usize * 2 + 1]; max_num as usize + 1];
        let mut ret = 2;

        for i in 0..nums.len() {
            for diff in -max_num..=max_num {
                if nums[i] - diff < 0 || nums[i] - diff > max_num {
                    dp[nums[i] as usize][(diff + max_num) as usize] = 1;
                } else {
                    dp[nums[i] as usize][(diff + max_num) as usize] =
                        dp[(nums[i] - diff) as usize][(diff + max_num) as usize] + 1;
                }
                ret = ret.max(dp[nums[i] as usize][(diff + max_num) as usize]);
            }
        }

        ret
    }
}
```
