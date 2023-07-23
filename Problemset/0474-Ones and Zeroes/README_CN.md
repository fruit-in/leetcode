# 474. 一和零
在计算机界中，我们总是追求用有限的资源获取最大的收益。

现在，假设你分别支配着 **m** 个 `0` 和 **n** 个 `1`。另外，还有一个仅包含 `0` 和 `1` 字符串的数组。

你的任务是使用给定的 **m** 个 `0` 和 **n** 个 `1` ，找到能拼出存在于数组中的字符串的最大数量。每个 `0` 和 `1` 至多被使用**一次**。

#### 注意:
1. 给定 `0` 和 `1` 的数量都不会超过 `100`。
2. 给定字符串数组的长度不会超过 `600`。

#### 示例 1:
<pre>
<strong>输入:</strong> Array = {"10", "0001", "111001", "1", "0"}, m = 5, n = 3
<strong>输出:</strong> 4
<strong>解释:</strong> 总共 4 个字符串可以通过 5 个 0 和 3 个 1 拼出，即 "10","0001","1","0" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> Array = {"10", "0", "1"}, m = 1, n = 1
<strong>输出:</strong> 2
<strong>解释:</strong> 你可以拼出 "10"，但之后就没有剩余数字了。更好的选择是拼出 "0" 和 "1" 。
</pre>

## 题解 (Rust)

### 1. 动态规划
```Rust
impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize + 1]; m as usize + 1];

        for s in strs {
            let zeros = s.chars().filter(|&c| c == '0').count();
            let ones = s.chars().filter(|&c| c == '1').count();
            for i in (zeros..=(m as usize)).rev() {
                for j in (ones..=(n as usize)).rev() {
                    dp[i][j] = dp[i][j].max(1 + dp[i - zeros][j - ones]);
                }
            }
        }

        dp[m as usize][n as usize]
    }
}
```
