# 2124. Check if All A's Appears Before All B's
Given a string `s` consisting of **only** the characters `'a'` and `'b'`, return `true` *if **every*** `'a'` *appears before **every*** `'b'` *in the string*. Otherwise, return `false`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "aaabbb"
<strong>Output:</strong> true
<strong>Explanation:</strong>
The 'a's are at indices 0, 1, and 2, while the 'b's are at indices 3, 4, and 5.
Hence, every 'a' appears before every 'b' and we return true.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "abab"
<strong>Output:</strong> false
<strong>Explanation:</strong>
There is an 'a' at index 2 and a 'b' at index 1.
Hence, not every 'a' appears before every 'b' and we return false.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "bbb"
<strong>Output:</strong> true
<strong>Explanation:</strong>
There are no 'a's, hence, every 'a' appears before every 'b' and we return true.
</pre>

#### Constraints:
* `1 <= s.length <= 100`
* `s[i]` is either `'a'` or `'b'`.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def checkString(self, s: str) -> bool:
        return all(s[i] <= s[i + 1] for i in range(len(s) - 1))
```
