# 132. 分割回文串 II
给你一个字符串 `s`，请你将 `s` 分割成一些子串，使每个子串都是回文串。

返回符合要求的 **最少分割次数** 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "aab"
<strong>输出:</strong> 1
<strong>解释:</strong> 只需一次分割就可将 s 分割成 ["aa","b"] 这样两个回文子串。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "a"
<strong>输出:</strong> 0
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "ab"
<strong>输出:</strong> 1
</pre>

#### 提示:
* `1 <= s.length <= 2000`
* `s` 仅由小写英文字母组成

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def minCut(self, s: str) -> int:
        @cache
        def isPalindrome(i: int, j: int) -> bool:
            return i >= j or (s[i] == s[j] and isPalindrome(i + 1, j - 1))

        @cache
        def dfs(j: int) -> int:
            if isPalindrome(0, j):
                return 0

            return min(dfs(i - 1) for i in range(1, j + 1) if isPalindrome(i, j)) + 1

        return dfs(len(s) - 1)
```
