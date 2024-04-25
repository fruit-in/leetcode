# 131. Palindrome Partitioning
Given a string `s`, partition `s` such that every substring of the partition is a **palindrome**. Return *all possible palindrome partitioning of* `s`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "aab"
<strong>Output:</strong> [["a","a","b"],["aa","b"]]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "a"
<strong>Output:</strong> [["a"]]
</pre>

#### Constraints:
* `1 <= s.length <= 16`
* `s` contains only lowercase English letters.

## Solutions (Python)

### 1. Solution
```Python
from functools import lru_cache


class Solution:
    @lru_cache()
    def partition(self, s: str) -> List[List[str]]:
        if len(s) == 0:
            return [[]]

        ret = []

        for i in range(1, len(s) + 1):
            if s[:i] == s[:i][::-1]:
                ret.extend([[s[:i]] + x for x in self.partition(s[i:])])

        return ret
```
