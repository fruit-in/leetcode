# 96. 不同的二叉搜索树
给定一个整数 *n*，求以 1 ... *n* 为节点组成的二叉搜索树有多少种？

#### 示例:
<pre>
<strong>输入:</strong> 3
<strong>输出:</strong> 5
<strong>解释:</strong>
给定 <i>n</i> = 3, 一共有 5 种不同结构的二叉搜索树:

   1         3     3      2      1
    \       /     /      / \      \
     3     2     1      1   3      2
    /     /       \                 \
   2     1         2                 3
</pre>

## 题解 (Rust)

### 1. 动态规划
```Rust
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1;

        for i in 1..=n {
            for j in 1..=i {
                dp[i] += dp[j - 1] * dp[i - j];
            }
        }

        dp[n]
    }
}
```
