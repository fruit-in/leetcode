# 1092. 最短公共超序列
给你两个字符串 `str1` 和 `str2`，返回同时以 `str1` 和 `str2` 作为 **子序列** 的最短字符串。如果答案不止一个，则可以返回满足条件的 **任意一个** 答案。

如果从字符串 `t` 中删除一些字符（也可能不删除），可以得到字符串 `s` ，那么 `s` 就是 `t` 的一个子序列。

#### 示例 1:
<pre>
<strong>输入:</strong> str1 = "abac", str2 = "cab"
<strong>输出:</strong> "cabac"
<strong>解释:</strong>
str1 = "abac" 是 "cabac" 的一个子串，因为我们可以删去 "cabac" 的第一个 "c"得到 "abac"。
str2 = "cab" 是 "cabac" 的一个子串，因为我们可以删去 "cabac" 末尾的 "ac" 得到 "cab"。
最终我们给出的答案是满足上述属性的最短字符串。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> str1 = "aaaaaaaa", str2 = "aaaaaaaa"
<strong>输出:</strong> "aaaaaaaa"
</pre>

#### 提示:
* `1 <= str1.length, str2.length <= 1000`
* `str1` 和 `str2` 都由小写英文字母组成。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def shortestCommonSupersequence(self, str1: str, str2: str) -> str:
        m, n = len(str1), len(str2)
        dp = [[0] * (n + 1) for _ in range(m + 1)]

        for i in range(m - 1, -1, -1):
            for j in range(n - 1, -1, -1):
                dp[i][j] = max(dp[i + 1][j], dp[i][j + 1])
                if str1[i] == str2[j]:
                    dp[i][j] = max(dp[i][j], dp[i + 1][j + 1] + 1)

        i = 0
        j = 0
        supersequence = []

        while i < m and j < n:
            maxlcs = max(dp[i + 1][j], dp[i][j + 1])
            if str1[i] == str2[j]:
                maxlcs = max(maxlcs, dp[i + 1][j + 1] + 1)

            if maxlcs == dp[i + 1][j]:
                supersequence.append(str1[i])
                i += 1
            elif maxlcs == dp[i][j + 1]:
                supersequence.append(str2[j])
                j += 1
            else:
                supersequence.append(str1[i])
                i += 1
                j += 1

        if i == m:
            supersequence.append(str2[j:])
        if j == n:
            supersequence.append(str1[i:])

        return ''.join(supersequence)
```
