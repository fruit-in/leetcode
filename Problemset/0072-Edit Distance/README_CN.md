# 72. 编辑距离
给你两个单词 `word1` 和 `word2`， *请返回将 `word1` 转换成 `word2` 所使用的最少操作数*  。

你可以对一个单词进行如下三种操作：
* 插入一个字符
* 删除一个字符
* 替换一个字符

#### 示例 1:
<pre>
<strong>输入:</strong> word1 = "horse", word2 = "ros"
<strong>输出:</strong> 3
<strong>解释:</strong>
horse -> rorse (将 'h' 替换为 'r')
rorse -> rose (删除 'r')
rose -> ros (删除 'e')
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> word1 = "intention", word2 = "execution"
<strong>输出:</strong> 5
<strong>解释:</strong>
intention -> inention (删除 't')
inention -> enention (将 'i' 替换为 'e')
enention -> exention (将 'n' 替换为 'x')
exention -> exection (将 'n' 替换为 'c')
exection -> execution (插入 'u')
</pre>

#### 提示:
* `0 <= word1.length, word2.length <= 500`
* `word1` 和 `word2` 由小写英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let mut dp = vec![vec![i32::MAX; word2.len() + 1]; word1.len() + 1];
        dp[0] = (0..=word2.len() as i32).collect();

        for i in 1..=word1.len() {
            dp[i][0] = i as i32;
            for j in 1..=word2.len() {
                if word1[i - 1] == word2[j - 1] {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - 1]);
                }
                dp[i][j] = dp[i][j]
                    .min(dp[i][j - 1] + 1)
                    .min(dp[i - 1][j] + 1)
                    .min(dp[i - 1][j - 1] + 1);
            }
        }

        dp[word1.len()][word2.len()]
    }
}
```
