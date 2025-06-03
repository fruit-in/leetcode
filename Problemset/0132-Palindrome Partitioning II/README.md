# 132. Palindrome Partitioning II
Given a string `s`, partition `s` such that every substring of the partition is a palindrome.

Return *the **minimum** cuts needed for a palindrome partitioning of* `s`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "aab"
<strong>Output:</strong> 1
<strong>Explanation:</strong> The palindrome partitioning ["aa","b"] could be produced using 1 cut.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "a"
<strong>Output:</strong> 0
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "ab"
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `1 <= s.length <= 2000`
* `s` consists of lowercase English letters only.

## Solutions (Python)

### 1. Solution
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
