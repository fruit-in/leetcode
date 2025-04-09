# 1639. 通过给定词典构造目标字符串的方案数
给你一个字符串列表 `words` 和一个目标字符串 `target` 。`words` 中所有字符串都 **长度相同**  。

你的目标是使用给定的 `words` 字符串列表按照下述规则构造 `target` ：
* 从左到右依次构造 `target` 的每一个字符。
* 为了得到 `target` 第 `i` 个字符（下标从 **0** 开始），当 `target[i] = words[j][k]` 时，你可以使用 `words` 列表中第 `j` 个字符串的第 `k` 个字符。
* 一旦你使用了 `words` 中第 `j` 个字符串的第 `k` 个字符，你不能再使用 `words` 字符串列表中任意单词的第 `x` 个字符（`x <= k`）。也就是说，所有单词下标小于等于 `k` 的字符都不能再被使用。
* 请你重复此过程直到得到目标字符串 `target` 。

**请注意**， 在构造目标字符串的过程中，你可以按照上述规定使用 `words` 列表中 **同一个字符串** 的 **多个字符** 。

请你返回使用 `words` 构造 `target` 的方案数。由于答案可能会很大，请对 <code>10<sup>9</sup> + 7</code> **取余** 后返回。

#### 示例 1:
<pre>
<strong>输入:</strong> words = ["acca","bbbb","caca"], target = "aba"
<strong>输出:</strong> 6
<strong>解释:</strong> 总共有 6 种方法构造目标串。
"aba" -> 下标为 0 ("acca")，下标为 1 ("bbbb")，下标为 3 ("caca")
"aba" -> 下标为 0 ("acca")，下标为 2 ("bbbb")，下标为 3 ("caca")
"aba" -> 下标为 0 ("acca")，下标为 1 ("bbbb")，下标为 3 ("acca")
"aba" -> 下标为 0 ("acca")，下标为 2 ("bbbb")，下标为 3 ("acca")
"aba" -> 下标为 1 ("caca")，下标为 2 ("bbbb")，下标为 3 ("acca")
"aba" -> 下标为 1 ("caca")，下标为 2 ("bbbb")，下标为 3 ("caca")
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> words = ["abba","baab"], target = "bab"
<strong>输出:</strong> 4
<strong>解释:</strong> 总共有 4 种不同形成 target 的方法。
"bab" -> 下标为 0 ("baab")，下标为 1 ("baab")，下标为 2 ("abba")
"bab" -> 下标为 0 ("baab")，下标为 1 ("baab")，下标为 3 ("baab")
"bab" -> 下标为 0 ("baab")，下标为 2 ("baab")，下标为 3 ("baab")
"bab" -> 下标为 1 ("abba")，下标为 2 ("baab")，下标为 3 ("baab")
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> words = ["abcd"], target = "abcd"
<strong>输出:</strong> 1
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> words = ["abab","baba","abba","baab"], target = "abba"
<strong>输出:</strong> 16
</pre>

#### 提示:
* `1 <= words.length <= 1000`
* `1 <= words[i].length <= 1000`
* `words` 中所有单词长度相同。
* `1 <= target.length <= 1000`
* `words[i]` 和 `target` 都仅包含小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let target = target
            .bytes()
            .map(|b| (b - b'a') as usize)
            .collect::<Vec<_>>();
        let mut count = vec![[0; 26]; words[0].len()];
        let mut dp = vec![vec![0; count.len() + 1]; target.len() + 1];
        dp[0] = vec![1_i64; count.len() + 1];

        for word in &words {
            let word = word.as_bytes();

            for i in 0..word.len() {
                count[i][(word[i] - b'a') as usize] += 1;
            }
        }

        for i in 1..=target.len() {
            for j in 1..=count.len() {
                dp[i][j] = dp[i][j - 1];
                dp[i][j] =
                    (dp[i][j] + dp[i - 1][j - 1] * count[j - 1][target[i - 1]]) % 1_000_000_007;
            }
        }

        dp[target.len()][count.len()] as i32
    }
}
```
