# 214. 最短回文串
给定一个字符串 ***s***，你可以通过在字符串前面添加字符将其转换为回文串。找到并返回可以用这种方式转换的最短回文串。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "aacecaaa"
<strong>输出:</strong> "aaacecaaa"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "abcd"
<strong>输出:</strong> "dcbabcd"
</pre>

#### 提示:
* <code>0 <= s.length <= 5 * 10<sup>4</sup></code>
* `s` 仅由小写英文字母组成

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def shortestPalindrome(self, s: str) -> str:
        if len(s) < 2:
            return s

        n = len(s)
        s += s[::-1]
        j = 0
        lps = [0] * (n * 2)

        for i in range(1, n * 2):
            while j > 0 and (s[i] != s[j] or j >= n):
                j = lps[j - 1]

            if s[i] == s[j]:
                j += 1
                lps[i] = j

        return s[n:n * 2 - lps[-1]] + s[:n]
```
