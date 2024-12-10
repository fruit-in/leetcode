# 1849. Splitting a String Into Descending Consecutive Values
You are given a string `s` that consists of only digits.

Check if we can split `s` into **two or more non-empty substrings** such that the **numerical values** of the substrings are in **descending order** and the **difference** between numerical values of every two **adjacent substrings** is equal to `1`.

* For example, the string `s = "0090089"` can be split into `["0090", "089"]` with numerical values `[90,89]`. The values are in descending order and adjacent values differ by `1`, so this way is valid.
* Another example, the string `s = "001"` can be split into `["0", "01"]`, `["00", "1"]`, or `["0", "0", "1"]`. However all the ways are invalid because they have numerical values `[0,1]`, `[0,1]`, and `[0,0,1]` respectively, all of which are not in descending order.

Return `true` *if it is possible to split* `s` *as described above, or* `false` *otherwise*.

A **substring** is a contiguous sequence of characters in a string.

#### Example 1:
<pre>
<strong>Input:</strong> s = "1234"
<strong>Output:</strong> false
<strong>Explanation:</strong> There is no valid way to split s.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "050043"
<strong>Output:</strong> true
<strong>Explanation:</strong> s can be split into ["05", "004", "3"] with numerical values [5,4,3].
The values are in descending order with adjacent values differing by 1.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "9080701"
<strong>Output:</strong> false
<strong>Explanation:</strong> There is no valid way to split s.
</pre>

#### Constraints:
* `1 <= s.length <= 20`
* `s` only consists of digits.

## Solutions (Python)

### 1. Solution
```Python
from functools import cache


class Solution:
    @cache
    def splitString(self, s: str, x: int = None) -> bool:
        if x is None:
            return any(self.splitString(s[i:], int(s[:i])) for i in range(1, len(s)))

        if s == "":
            return True

        return any(x - 1 == int(s[:i]) and self.splitString(s[i:], int(s[:i])) for i in range(1, len(s) + 1))
```
