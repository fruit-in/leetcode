# 2131. 连接两字母单词得到的最长回文串
给你一个字符串数组 `words` 。`words` 中每个元素都是一个包含 **两个** 小写英文字母的单词。

请你从 `words` 中选择一些元素并按 **任意顺序** 连接它们，并得到一个 **尽可能长的回文串** 。每个元素 **至多** 只能使用一次。

请你返回你能得到的最长回文串的 **长度** 。如果没办法得到任何一个回文串，请你返回 `0` 。

**回文串** 指的是从前往后和从后往前读一样的字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> words = ["lc","cl","gg"]
<strong>输出:</strong> 6
<strong>解释:</strong> 一个最长的回文串为 "lc" + "gg" + "cl" = "lcggcl" ，长度为 6 。
"clgglc" 是另一个可以得到的最长回文串。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> words = ["ab","ty","yt","lc","cl","ab"]
<strong>输出:</strong> 8
<strong>解释:</strong> 最长回文串是 "ty" + "lc" + "cl" + "yt" = "tylcclyt" ，长度为 8 。
"lcyttycl" 是另一个可以得到的最长回文串。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> words = ["cc","ll","xx"]
<strong>输出:</strong> 2
<strong>解释:</strong> 最长回文串是 "cc" ，长度为 2 。
"ll" 是另一个可以得到的最长回文串。"xx" 也是。
</pre>

#### 提示:
* <code>1 <= words.length <= 10<sup>5</sup></code>
* `words[i].length == 2`
* `words[i]` 仅包含小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut mat = [[0; 26]; 26];
        let mut flag = 0;
        let mut ret = 0;

        for word in words {
            mat[(word.as_bytes()[0] - b'a') as usize][(word.as_bytes()[1] - b'a') as usize] += 1;
        }

        for i in 0..26 {
            flag |= mat[i][i] % 2;
            ret += mat[i][i] / 2 * 4;
            for j in (i + 1)..26 {
                ret += 4 * mat[i][j].min(mat[j][i])
            }
        }

        ret + 2 * flag
    }
}
```
