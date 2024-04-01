# 798. 得分最高的最小轮调
给你一个数组 `nums`，我们可以将它按一个非负整数 `k` 进行轮调，这样可以使数组变为 `[nums[k], nums[k + 1], ... nums[nums.length - 1], nums[0], nums[1], ..., nums[k-1]]` 的形式。此后，任何值小于或等于其索引的项都可以记作一分。

* 例如，数组为 `nums = [2,4,1,3,0]`，我们按 `k = 2` 进行轮调后，它将变成 `[1,3,0,2,4]`。这将记为 `3` 分，因为 `1 > 0` [不计分]、`3 > 1` [不计分]、`0 <= 2` [计 1 分]、`2 <= 3` [计 1 分]，`4 <= 4` [计 1 分]。

在所有可能的轮调中，返回我们所能得到的最高分数对应的轮调下标 `k` 。如果有多个答案，返回满足条件的最小的下标 `k` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,3,1,4,0]
<strong>输出:</strong> 3
<strong>解释:</strong>
下面列出了每个 k 的得分：
k = 0,  nums = [2,3,1,4,0],    score 2
k = 1,  nums = [3,1,4,0,2],    score 3
k = 2,  nums = [1,4,0,2,3],    score 3
k = 3,  nums = [4,0,2,3,1],    score 4
k = 4,  nums = [0,2,3,1,4],    score 3
所以我们应当选择 k = 3，得分最高。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,3,0,2,4]
<strong>输出:</strong> 0
<strong>解释:</strong>
nums 无论怎么变化总是有 3 分。
所以我们将选择最小的 k，即 0。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* `0 <= nums[i] < nums.length`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn best_rotation(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut prefix_sum = vec![0; n + 1];
        let mut max_score = 0;
        let mut ret = 0;

        for i in 0..n {
            prefix_sum[(n + i - nums[i] as usize) % n] += 1;
            prefix_sum[i] -= 1;
        }

        for i in (0..n).rev() {
            prefix_sum[i] += prefix_sum[i + 1];
            if max_score <= prefix_sum[i] {
                max_score = prefix_sum[i];
                ret = i;
            }
        }

        ret as i32
    }
}
```
