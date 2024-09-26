# 650. 两个键的键盘
最初记事本上只有一个字符 `'A'` 。你每次可以对这个记事本进行两种操作：

* `Copy All`（复制全部）：复制这个记事本中的所有字符（不允许仅复制部分字符）。
* `Paste`（粘贴）：粘贴 **上一次** 复制的字符。

给你一个数字 `n` ，你需要使用最少的操作次数，在记事本上输出 **恰好** `n` 个 `'A'` 。返回能够打印出 `n` 个 `'A'` 的最少操作次数。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 3
<strong>输出:</strong> 3
<strong>解释:</strong>
最初, 只有一个字符 'A'。
第 1 步, 使用 Copy All 操作。
第 2 步, 使用 Paste 操作来获得 'AA'。
第 3 步, 使用 Paste 操作来获得 'AAA'。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 1
<strong>输出:</strong> 0
</pre>

#### 提示:
* `1 <= n <= 1000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let mut dp = vec![usize::MAX; n as usize + 1];
        dp[1] = 0;

        for i in 1..dp.len() {
            for j in (i..dp.len()).step_by(i) {
                dp[j] = dp[j].min(dp[i] + j / i);
            }
        }

        dp[n as usize] as i32
    }
}
```
