# 2028. 找出缺失的观测数据
现有一份 `n + m` 次投掷单个 **六面** 骰子的观测数据，骰子的每个面从 `1` 到 `6` 编号。观测数据中缺失了 `n` 份，你手上只拿到剩余 m 次投掷的数据。幸好你有之前计算过的这 `n + m` 次投掷数据的 **平均值** 。

给你一个长度为 `m` 的整数数组 `rolls` ，其中 `rolls[i]` 是第 `i` 次观测的值。同时给你两个整数 `mean` 和 `n` 。

返回一个长度为 `n` 的数组，包含所有缺失的观测数据，且满足这 `n + m` 次投掷的 **平均值** 是 `mean` 。如果存在多组符合要求的答案，只需要返回其中任意一组即可。如果不存在答案，返回一个空数组。

`k` 个数字的 **平均值** 为这些数字求和后再除以 `k` 。

注意 `mean` 是一个整数，所以 `n + m` 次投掷的总和需要被 `n + m` 整除。

#### 示例 1:
<pre>
<strong>输入:</strong> rolls = [3,2,4,3], mean = 4, n = 2
<strong>输出:</strong> [6,6]
<strong>解释:</strong> 所有 n + m 次投掷的平均值是 (3 + 2 + 4 + 3 + 6 + 6) / 6 = 4 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> rolls = [1,5,6], mean = 3, n = 4
<strong>输出:</strong> [2,3,2,2]
<strong>解释:</strong> 所有 n + m 次投掷的平均值是 (1 + 5 + 6 + 2 + 3 + 2 + 2) / 7 = 3 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> rolls = [1,2,3,4], mean = 6, n = 4
<strong>输出:</strong> []
<strong>解释:</strong> 无论丢失的 4 次数据是什么，平均值都不可能是 6 。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> rolls = [1], mean = 3, n = 1
<strong>输出:</strong> [5]
<strong>解释:</strong> 所有 n + m 次投掷的平均值是 (1 + 5) / 2 = 3 。
</pre>

#### 提示:
* `m == rolls.length`
* <code>1 <= n, m <= 10<sup>5</sup></code>
* `1 <= rolls[i], mean <= 6`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let mut n_sum = mean * (rolls.len() as i32 + n) - rolls.into_iter().sum::<i32>();
        let mut ret = vec![];

        if n_sum < n || n_sum > 6 * n {
            return ret;
        }

        for i in (0..n).rev() {
            let x = (n_sum - 6 * i).max(1);
            n_sum -= x;
            ret.push(x);
        }

        ret
    }
}
```
