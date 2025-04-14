# 1771. Maximize Palindrome Length From Subsequences
You are given two strings, `word1` and `word2`. You want to construct a string in the following manner:
* Choose some **non-empty** subsequence `subsequence1` from `word1`.
* Choose some **non-empty** subsequence `subsequence2` from `word2`.
* Concatenate the subsequences: `subsequence1 + subsequence2`, to make the string.

Return *the **length** of the longest **palindrome** that can be constructed in the described manner*. If no palindromes can be constructed, return `0`.

A **subsequence** of a string `s` is a string that can be made by deleting some (possibly none) characters from `s` without changing the order of the remaining characters.

A **palindrome** is a string that reads the same forward as well as backward.

#### Example 1:
<pre>
<strong>Input:</strong> word1 = "cacb", word2 = "cbba"
<strong>Output:</strong> 5
<strong>Explanation:</strong> Choose "ab" from word1 and "cba" from word2 to make "abcba", which is a palindrome.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> word1 = "ab", word2 = "ab"
<strong>Output:</strong> 3
<strong>Explanation:</strong> Choose "ab" from word1 and "a" from word2 to make "aba", which is a palindrome.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> word1 = "aa", word2 = "bb"
<strong>Output:</strong> 0
<strong>Explanation:</strong> You cannot construct a palindrome from the described method, so return 0.
</pre>

#### Constraints:
* `1 <= word1.length, word2.length <= 1000`
* `word1` and `word2` consist of lowercase English letters.

## Solutions (Python)

### 1. Solution
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
