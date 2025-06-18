# 214. Shortest Palindrome
You are given a string `s`. You can convert s to a palindrome by adding characters in front of it.

Return *the shortest palindrome you can find by performing this transformation*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "aacecaaa"
<strong>Output:</strong> "aaacecaaa"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "abcd"
<strong>Output:</strong> "dcbabcd"
</pre>

#### Constraints:
* <code>0 <= s.length <= 5 * 10<sup>4</sup></code>
* `s` consists of lowercase English letters only.

## Solutions (Python)

### 1. Solution
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
