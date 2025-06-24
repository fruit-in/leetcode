# 2430. 对字母串可执行的最大删除数
给你一个仅由小写英文字母组成的字符串 `s` 。在一步操作中，你可以：
* 删除 **整个字符串** `s` ，或者
* 对于满足 `1 <= i <= s.length / 2` 的任意 `i` ，如果 `s` 中的 前 `i` 个字母和接下来的 `i` 个字母 **相等** ，删除 **前** `i` 个字母。

例如，如果 `s = "ababc"` ，那么在一步操作中，你可以删除 `s` 的前两个字母得到 `"abc"` ，因为 `s` 的前两个字母和接下来的两个字母都等于 `"ab"` 。

返回删除 `s` 所需的最大操作数。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "abcabcdabc"
<strong>输出:</strong> 2
<strong>解释:</strong>
- 删除前 3 个字母（"abc"），因为它们和接下来 3 个字母相等。现在，s = "abcdabc"。
- 删除全部字母。
一共用了 2 步操作，所以返回 2 。可以证明 2 是所需的最大操作数。
注意，在第二步操作中无法再次删除 "abc" ，因为 "abc" 的下一次出现并不是位于接下来的 3 个字母。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "aaabaab"
<strong>输出:</strong> 4
<strong>解释:</strong>
- 删除第一个字母（"a"），因为它和接下来的字母相等。现在，s = "aabaab"。
- 删除前 3 个字母（"aab"），因为它们和接下来 3 个字母相等。现在，s = "aab"。
- 删除第一个字母（"a"），因为它和接下来的字母相等。现在，s = "ab"。
- 删除全部字母。
一共用了 4 步操作，所以返回 4 。可以证明 4 是所需的最大操作数。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "aaaaa"
<strong>输出:</strong> 5
<strong>解释:</strong> 在每一步操作中，都可以仅删除 s 的第一个字母。
</pre>

#### 提示:
* `1 <= s.length <= 4000`
* `s` 仅由小写英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn delete_string(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut lcp = vec![vec![0; n]; n];
        let mut dp = vec![1; n];

        for i in (0..n - 1).rev() {
            for j in (i + 1..n).rev() {
                if s[i] == s[j] {
                    lcp[i][j] = 1;
                    if j + 1 < n {
                        lcp[i][j] += lcp[i + 1][j + 1];
                    }
                }
            }
        }

        for i in (0..n - 1).rev() {
            for j in i + 1..=(n + i) / 2 {
                if lcp[i][j] >= j - i {
                    dp[i] = dp[i].max(1 + dp[j]);
                }
            }
        }

        dp[0]
    }
}
```
