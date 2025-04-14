# 1771. 由子序列构造的最长回文串的长度
给你两个字符串 `word1` 和 `word2` ，请你按下述方法构造一个字符串：
* 从 `word1` 中选出某个 **非空** 子序列 `subsequence1` 。
* 从 `word2` 中选出某个 **非空** 子序列 `subsequence2` 。
* 连接两个子序列 `subsequence1 + subsequence2` ，得到字符串。

返回可按上述方法构造的最长 **回文串** 的 **长度** 。如果无法构造回文串，返回 `0` 。

字符串 `s` 的一个 **子序列** 是通过从 `s` 中删除一些（也可能不删除）字符而不更改其余字符的顺序生成的字符串。

**回文串** 是正着读和反着读结果一致的字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> word1 = "cacb", word2 = "cbba"
<strong>输出:</strong> 5
<strong>解释:</strong> 从 word1 中选出 "ab" ，从 word2 中选出 "cba" ，得到回文串 "abcba" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> word1 = "ab", word2 = "ab"
<strong>输出:</strong> 3
<strong>解释:</strong> 从 word1 中选出 "ab" ，从 word2 中选出 "a" ，得到回文串 "aba" 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> word1 = "aa", word2 = "bb"
<strong>输出:</strong> 0
<strong>解释:</strong> 无法按题面所述方法构造回文串，所以返回 0 。
</pre>

#### 提示:
* `1 <= word1.length, word2.length <= 1000`
* `word1` 和 `word2` 由小写英文字母组成

## 题解 (Python)

### 1. 题解
```Python
from functools import cache


class Solution:
    def longestPalindrome(self, word1: str, word2: str) -> int:
        @cache
        def longestSubPalindrome(i: int, j: int) -> int:
            if i > j:
                return 0
            if i == j:
                return 1

            ret = max(longestSubPalindrome(i, j - 1),
                      longestSubPalindrome(i + 1, j))
            if word[i] == word[j]:
                ret = max(ret, 2 + longestSubPalindrome(i + 1, j - 1))

            return ret

        word = word1 + word2
        first = [[-1, -1] for _ in range(26)]
        ret = 0

        for i in range(len(word1) - 1, -1, -1):
            first[ord(word1[i]) - 97][0] = i
        for i in range(len(word1), len(word)):
            first[ord(word[i]) - 97][1] = i

        for i, j in first:
            if i >= 0 and j >= 0:
                ret = max(ret, 2 + longestSubPalindrome(i + 1, j - 1))

        return ret
```
