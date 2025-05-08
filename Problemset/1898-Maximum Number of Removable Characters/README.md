# 1898. Maximum Number of Removable Characters
You are given two strings `s` and `p` where `p` is a **subsequence** of `s`. You are also given a **distinct 0-indexed** integer array `removable` containing a subset of indices of `s` (`s` is also **0-indexed**).

You want to choose an integer `k` (`0 <= k <= removable.length`) such that, after removing `k` characters from `s` using the **first** `k` indices in `removable`, `p` is still a **subsequence** of `s`. More formally, you will mark the character at `s[removable[i]]` for each `0 <= i < k`, then remove all marked characters and check if `p` is still a subsequence.

Return *the **maximum*** `k` *you can choose such that* `p` *is still a **subsequence** of* `s` *after the removals*.

A **subsequence** of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.

#### Example 1:
<pre>
<strong>Input:</strong> s = "abcacb", p = "ab", removable = [3,1,0]
<strong>Output:</strong> 2
<strong>Explanation:</strong> After removing the characters at indices 3 and 1, "abcacb" becomes "accb".
"ab" is a subsequence of "accb".
If we remove the characters at indices 3, 1, and 0, "abcacb" becomes "ccb", and "ab" is no longer a subsequence.
Hence, the maximum k is 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "abcbddddd", p = "abcd", removable = [3,2,1,4,5,6]
<strong>Output:</strong> 1
<strong>Explanation:</strong> After removing the character at index 3, "abcbddddd" becomes "abcddddd".
"abcd" is a subsequence of "abcddddd".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "abcab", p = "abc", removable = [0,1,2,3,4]
<strong>Output:</strong> 0
<strong>Explanation:</strong> If you remove the first index in the array removable, "abc" is no longer a subsequence.
</pre>

#### Constraints:
* <code>1 <= p.length <= s.length <= 10<sup>5</sup></code>
* `0 <= removable.length < s.length`
* `0 <= removable[i] < s.length`
* `p` is a **subsequence** of `s`.
* `s` and `p` both consist of lowercase English letters.
* The elements in `removable` are **distinct**.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def maximumRemovals(self, s: str, p: str, removable: List[int]) -> int:
        def isNotSubsequence(k: int) -> int:
            removed = set(removable[:k])
            i = 0

            for j in range(len(s)):
                if j not in removed and p[i] == s[j]:
                    i += 1
                if i == len(p):
                    break

            return i < len(p)

        return bisect.bisect(range(len(removable) + 1), False, key=isNotSubsequence) - 1
```
