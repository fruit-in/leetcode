# 459. Repeated Substring Pattern
Given a non-empty string check if it can be constructed by taking a substring of it and appending multiple copies of the substring together. You may assume the given string consists of lowercase English letters only and its length will not exceed 10000.

#### Example 1:
<pre>
<strong>Input:</strong> "abab"
<strong>Output:</strong> True
<strong>Explanation:</strong> It's the substring "ab" twice.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "aba"
<strong>Output:</strong> False
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> "abcabcabcabc"
<strong>Output:</strong> True
<strong>Explanation:</strong> It's the substring "abc" four times. (And the substring "abcabc" twice.)
</pre>

## Solutions (Python)

### 1. Solution
```Python3
class Solution:
    def repeatedSubstringPattern(self, s: str) -> bool:
        length = len(s)
        for i in filter(lambda x: length % x == 0, range(1, length // 2 + 1)):
            if s[:i] * (length // i) == s:
                return True
        return False
```

### 2. Solution
```Python3
class Solution:
    def repeatedSubstringPattern(self, s: str) -> bool:
        return s in (s + s)[1:-1]
```
