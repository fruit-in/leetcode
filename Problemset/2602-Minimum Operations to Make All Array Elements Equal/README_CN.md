# 2602. 使数组元素全部相等的最少操作次数
给你一个正整数数组 `nums` 。

同时给你一个长度为 `m` 的整数数组 `queries` 。第 `i` 个查询中，你需要将 `nums` 中所有元素变成 `queries[i]` 。你可以执行以下操作 **任意** 次：

* 将数组里一个元素 **增大** 或者 **减小** `1` 。

请你返回一个长度为 `m` 的数组 `answer` ，其中 `answer[i]`是将 `nums` 中所有元素变成 `queries[i]` 的 **最少** 操作次数。

**注意**，每次查询后，数组变回最开始的值。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,1,6,8], queries = [1,5]
<strong>输出:</strong> [14,10]
<strong>解释:</strong> 第一个查询，我们可以执行以下操作：
- 将 nums[0] 减小 2 次，nums = [1,1,6,8] 。
- 将 nums[2] 减小 5 次，nums = [1,1,1,8] 。
- 将 nums[3] 减小 7 次，nums = [1,1,1,1] 。
第一个查询的总操作次数为 2 + 5 + 7 = 14 。
第二个查询，我们可以执行以下操作：
- 将 nums[0] 增大 2 次，nums = [5,1,6,8] 。
- 将 nums[1] 增大 4 次，nums = [5,5,6,8] 。
- 将 nums[2] 减小 1 次，nums = [5,5,5,8] 。
- 将 nums[3] 减小 3 次，nums = [5,5,5,5] 。
第二个查询的总操作次数为 2 + 4 + 1 + 3 = 10 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,9,6,3], queries = [10]
<strong>输出:</strong> [20]
<strong>解释:</strong> 我们可以将数组中所有元素都增大到 10 ，总操作次数为 8 + 1 + 4 + 7 = 20 。
</pre>

#### 提示:
* `n == nums.length`
* `m == queries.length`
* <code>1 <= n, m <= 10<sup>5</sup></code>
* <code>1 <= nums[i], queries[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_operations(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64> {
        let (n, m) = (nums.len(), queries.len());
        let mut nums = nums;
        let mut prefix_sum = vec![0; n];
        let mut answer = vec![0; m];

        nums.sort_unstable();
        prefix_sum[0] = nums[0] as i64;

        for i in 1..n {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i] as i64;
        }

        for i in 0..queries.len() {
            let (j, x) = match nums.binary_search(&queries[i]) {
                Ok(0) | Err(0) => (0, 0),
                Ok(k) | Err(k) => (k as i64, prefix_sum[k - 1]),
            };

            answer[i] = (2 * j - n as i64) * queries[i] as i64 + prefix_sum[n - 1] - 2 * x;
        }

        answer
    }
}
```
