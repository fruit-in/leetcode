# 22. 括号生成
数字 *n* 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 **有效的** 括号组合。

#### 示例:
<pre>
<strong>输入:</strong> n = 3
<strong>输出:</strong> [
       "((()))",
       "(()())",
       "(())()",
       "()(())",
       "()()()"
     ]
</pre>

## 题解 (Rust)

### 1. 动态规划
```Rust
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut dp = vec![vec![vec![]; n + 1]; n + 1];
        dp[0][0].push(String::new());

        for i in 0..=n {
            for j in 0..=i {
                if i == n && j == i {
                    return dp[i].pop().unwrap()
                }

                while let Some(s) = dp[i][j].pop() {
                    if i < n {
                        dp[i + 1][j].push(s.clone() + "(");
                    }
                    if i > j {
                        dp[i][j + 1].push(s + ")");
                    }
                }
            }
        }

        vec![]
    }
}
```
