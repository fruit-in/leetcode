# 1593. Split a String Into the Max Number of Unique Substrings
Given a string `s`, return *the maximum number of unique substrings that the given string can be split into*.

You can split string `s` into any list of **non-empty substrings**, where the concatenation of the substrings forms the original string. However, you must split the substrings such that all of them are **unique**.

A **substring** is a contiguous sequence of characters within a string.

#### Example 1:
<pre>
<strong>Input:</strong> s = "ababccc"
<strong>Output:</strong> 5
<strong>Explanation:</strong> One way to split maximally is ['a', 'b', 'ab', 'c', 'cc']. Splitting like ['a', 'b', 'a', 'b', 'c', 'cc'] is not valid as you have 'a' and 'b' multiple times.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "aba"
<strong>Output:</strong> 2
<strong>Explanation:</strong> One way to split maximally is ['a', 'ba'].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "aa"
<strong>Output:</strong> 1
<strong>Explanation:</strong> It is impossible to split the string any further.
</pre>

#### Constraints:
* `1 <= s.length <= 16`
* `s` contains only lower case English letters.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def maxUniqueSplit(self, s: str) -> int:
        ret = 1

        def dfs(i: int, subs: Set[str]) -> None:
            nonlocal ret
            if i == len(s):
                ret = max(ret, len(subs))
            if len(subs) + len(s) - i <= ret:
                return

            for j in range(i, len(s)):
                if s[i:j + 1] not in subs:
                    dfs(j + 1, subs | {s[i:j + 1]})

        dfs(0, set())

        return ret
```
