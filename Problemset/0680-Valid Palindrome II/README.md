# 680. Valid Palindrome II
Given a non-empty string ```s```, you may delete **at most** one character. Judge whether you can make it a palindrome.

#### Example 1:
<pre>
<strong>Input:</strong> "aba"
<strong>Output:</strong> True
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "abca"
<strong>Output:</strong> True
<strong>Explanation:</strong> You could delete the character 'c'.
</pre>

#### Note:
1. The string will only contain lowercase characters a-z. The maximum length of the string is 50000.

## Solutions (Python)

### 1. Two Pointers
```Python3
class Solution:
    def validPalindrome(self, s: str) -> bool:
        def isPalindrome(l: int, r: int) -> bool:
            return all(s[l + i] == s[r - i] for i in range((r - l + 1) // 2))

        l, r = 0, len(s) - 1
        while l < r:
            if s[l] != s[r]:
                return isPalindrome(l + 1, r) or isPalindrome(l, r - 1)

            l += 1
            r -= 1

        return True
```
