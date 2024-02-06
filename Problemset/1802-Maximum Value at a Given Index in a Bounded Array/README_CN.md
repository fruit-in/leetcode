# 1802. 有界数组中指定下标处的最大值
给你三个正整数 `n`、`index` 和 `maxSum` 。你需要构造一个同时满足下述所有条件的数组 `nums`（下标 **从 0 开始** 计数）：

* `nums.length == n`
* `nums[i]` 是 **正整数** ，其中 `0 <= i < n`
* `abs(nums[i] - nums[i+1]) <= 1` ，其中 `0 <= i < n-1`
* `nums` 中所有元素之和不超过 `maxSum`
* `nums[index]` 的值被 **最大化**

返回你所构造的数组中的 `nums[index]` 。

注意：abs(x) 等于 x 的前提是 x >= 0 ；否则，abs(x) 等于 -x 。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 4, index = 2,  maxSum = 6
<strong>输出:</strong> 2
<strong>解释:</strong> 数组 [1,1,2,1] 和 [1,2,2,1] 满足所有条件。不存在其他在指定下标处具有更大值的有效数组。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 6, index = 1,  maxSum = 10
<strong>输出:</strong> 3
</pre>

#### 提示:
* <code>1 <= n <= maxSum <= 10<sup>9</sup></code>
* `0 <= index < n`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        if index < n - index - 1 {
            return Self::max_value(n, n - index - 1, max_sum);
        }

        let n = n as i64;
        let index = index as i64;
        let max_sum = max_sum as i64 - n;
        let mut min = 0;
        let mut max = max_sum;

        while min < max {
            let x = (min + max + 1) / 2;
            let sum = if x > index {
                ((2 * x - index) * (index + 1) + (2 * x - n + index) * (n - 1 - index)) / 2
            } else if x > n - 1 - index {
                ((1 + x) * x + (2 * x - n + index) * (n - 1 - index)) / 2
            } else {
                x * x
            };

            if sum <= max_sum {
                min = x;
            } else {
                max = x - 1;
            }
        }

        min as i32 + 1
    }
}
```
