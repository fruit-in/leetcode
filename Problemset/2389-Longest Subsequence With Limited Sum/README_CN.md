# 2389. 和有限的最长子序列
给你一个长度为 `n` 的整数数组 `nums` ，和一个长度为 `m` 的整数数组 `queries` 。

返回一个长度为 `m` 的数组 `answer` ，其中 `answer[i]` 是 `nums` 中 元素之和小于等于 `queries[i]` 的 **子序列** 的 **最大** 长度  。

**子序列** 是由一个数组删除某些元素（也可以不删除）但不改变剩余元素顺序得到的一个数组。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [4,5,2,1], queries = [3,10,21]
<strong>输出:</strong> [2,3,4]
<strong>解释:</strong> queries 对应的 answer 如下：
- 子序列 [2,1] 的和小于或等于 3 。可以证明满足题目要求的子序列的最大长度是 2 ，所以 answer[0] = 2 。
- 子序列 [4,5,1] 的和小于或等于 10 。可以证明满足题目要求的子序列的最大长度是 3 ，所以 answer[1] = 3 。
- 子序列 [4,5,2,1] 的和小于或等于 21 。可以证明满足题目要求的子序列的最大长度是 4 ，所以 answer[2] = 4 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,3,4,5], queries = [1]
<strong>输出:</strong> [0]
<strong>解释:</strong> 空子序列是唯一一个满足元素和小于或等于 1 的子序列，所以 answer[0] = 0 。
</pre>

#### 提示:
* `n == nums.length`
* `m == queries.length`
* `1 <= n, m <= 1000`
* <code>1 <= nums[i], queries[i] <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut answer = vec![0; queries.len()];

        nums.sort_unstable();
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }

        for i in 0..queries.len() {
            answer[i] = match nums.binary_search(&queries[i]) {
                Ok(j) => j as i32 + 1,
                Err(j) => j as i32,
            };
        }

        answer
    }
}
```
