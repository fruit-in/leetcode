# 474. Ones and Zeroes
Given an array, `strs`, with strings consisting of only `0s` and `1s`. Also two integers `m` and `n`.

Now your task is to find the maximum number of strings that you can form with given **m** `0s` and **n** `1s`. Each `0` and `1` can be used at most **once**.

#### Example 1:
<pre>
<strong>Input:</strong> strs = ["10","0001","111001","1","0"], m = 5, n = 3
<strong>Output:</strong> 4
<strong>Explanation:</strong> This are totally 4 strings can be formed by the using of 5 0s and 3 1s, which are "10","0001","1","0".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> strs = ["10","0","1"], m = 1, n = 1
<strong>Output:</strong> 2
<strong>Explanation:</strong> You could form "10", but then you'd have nothing left. Better form "0" and "1".
</pre>

#### Constraints:
* `1 <= strs.length <= 600`
* `1 <= strs[i].length <= 100`
* `strs[i]` consists only of digits '0' and '1'.
* `1 <= m, n <= 100`

## Solutions (Rust)

### 1. Dynamic Programming
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
