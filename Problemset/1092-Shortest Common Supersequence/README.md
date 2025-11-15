# 1092. Shortest Common Supersequence
Given two strings `str1` and `str2`, return *the shortest string that has both* `str1` *and* `str2` *as **subsequences***. If there are multiple valid strings, return **any** of them.

A string `s` is a **subsequence** of string `t` if deleting some number of characters from `t` (possibly `0`) results in the string `s`.

#### Example 1:
<pre>
<strong>Input:</strong> str1 = "abac", str2 = "cab"
<strong>Output:</strong> "cabac"
<strong>Explanation:</strong>
str1 = "abac" is a subsequence of "cabac" because we can delete the first "c".
str2 = "cab" is a subsequence of "cabac" because we can delete the last "ac".
The answer provided is the shortest such string that satisfies these properties.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> str1 = "aaaaaaaa", str2 = "aaaaaaaa"
<strong>Output:</strong> "aaaaaaaa"
</pre>

#### Constraints:
* `1 <= str1.length, str2.length <= 1000`
* `str1` and `str2` consist of lowercase English letters.

## Solutions (Python)

### 1. Solution
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
