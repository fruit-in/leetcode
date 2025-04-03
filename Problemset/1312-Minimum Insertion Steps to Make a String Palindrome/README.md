# 1312. Minimum Insertion Steps to Make a String Palindrome
Given a string `s`. In one step you can insert any character at any index of the string.

Return *the minimum number of steps* to make `s` palindrome.

A **Palindrome String** is one that reads the same backward as well as forward.

#### Example 1:
<pre>
<strong>Input:</strong> s = "zzazz"
<strong>Output:</strong> 0
<strong>Explanation:</strong> The string "zzazz" is already palindrome we do not need any insertions.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "mbadm"
<strong>Output:</strong> 2
<strong>Explanation:</strong> String can be "mbdadbm" or "mdbabdm".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "leetcode"
<strong>Output:</strong> 5
<strong>Explanation:</strong> Inserting 5 characters the string becomes "leetcodocteel".
</pre>

#### Constraints:
* `1 <= s.length <= 500`
* `s` consists of lowercase English letters.

## Solutions (Python)

### 1. Solution
```Python
from functools import cache


class Solution:
    def minInsertions(self, s: str) -> int:
        @cache
        def minInsertionsSub(i: int, j: int) -> int:
            if i >= j:
                return 0

            ret = 1 + minInsertionsSub(i + 1, j)
            ret = min(ret, 1 + minInsertionsSub(i, j - 1))
            if s[i] == s[j]:
                ret = min(ret, minInsertionsSub(i + 1, j - 1))

            return ret

        return minInsertionsSub(0, len(s) - 1)
```
