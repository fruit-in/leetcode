# 1278. 分割回文串 III
给你一个由小写字母组成的字符串 `s`，和一个整数 `k`。

请你按下面的要求分割字符串：
* 首先，你可以将 `s` 中的部分字符修改为其他的小写英文字母。
* 接着，你需要把 `s` 分割成 `k` 个非空且不相交的子串，并且每个子串都是回文串。

请返回以这种方式分割字符串所需修改的最少字符数。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "abc", k = 2
<strong>输出:</strong> 1
<strong>解释:</strong> 你可以把字符串分割成 "ab" 和 "c"，并修改 "ab" 中的 1 个字符，将它变成回文串。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "aabbc", k = 3
<strong>输出:</strong> 0
<strong>解释:</strong> 你可以把字符串分割成 "aa"、"bb" 和 "c"，它们都是回文串。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "leetcode", k = 8
<strong>输出:</strong> 0
</pre>

#### 提示:
* `1 <= k <= s.length <= 100`
* `s` 中只含有小写英文字母。

## 题解 (Python)

### 1. 题解
```Python
from functools import cache


class Solution:
    def palindromePartition(self, s: str, k: int) -> int:
        @cache
        def palindromeChange(i: int, j: int) -> int:
            if i >= j:
                return 0

            return palindromeChange(i + 1, j - 1) + (s[i] != s[j])

        @cache
        def subPartition(i: int, k: int) -> int:
            if k == 1:
                return palindromeChange(i, len(s) - 1)

            return min(palindromeChange(i, j) + subPartition(j + 1, k - 1) for j in range(i, len(s) - k + 1))

        return subPartition(0, k)
```
