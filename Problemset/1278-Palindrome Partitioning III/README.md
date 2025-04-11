# 1278. Palindrome Partitioning III
You are given a string `s` containing lowercase letters and an integer `k`. You need to :
* First, change some characters of `s` to other lowercase English letters.
* Then divide `s` into `k` non-empty disjoint substrings such that each substring is a palindrome.

Return *the minimal number of characters that you need to change to divide the string*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "abc", k = 2
<strong>Output:</strong> 1
<strong>Explanation:</strong> You can split the string into "ab" and "c", and change 1 character in "ab" to make it palindrome.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "aabbc", k = 3
<strong>Output:</strong> 0
<strong>Explanation:</strong> You can split the string into "aa", "bb" and "c", all of them are palindrome.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "leetcode", k = 8
<strong>Output:</strong> 0
</pre>

#### Constraints:
* `1 <= k <= s.length <= 100`.
* `s` only contains lowercase English letters.

## Solutions (Python)

### 1. Solution
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
