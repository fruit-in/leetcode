# 854. K-Similar Strings
Strings `s1` and `s2` are `k`**-similar** (for some non-negative integer `k`) if we can swap the positions of two letters in `s1` exactly `k` times so that the resulting string equals `s2`.

Given two anagrams `s1` and `s2`, return the smallest `k` for which `s1` and `s2` are `k`**-similar**.

#### Example 1:
<pre>
<strong>Input:</strong> s1 = "ab", s2 = "ba"
<strong>Output:</strong> 1
<strong>Explanation:</strong> The two string are 1-similar because we can use one swap to change s1 to s2: "ab" --> "ba".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s1 = "abc", s2 = "bca"
<strong>Output:</strong> 2
<strong>Explanation:</strong> The two strings are 2-similar because we can use two swaps to change s1 to s2: "abc" --> "bac" --> "bca".
</pre>

#### Constraints:
* `1 <= s1.length <= 20`
* `s2.length == s1.length`
* `s1` and `s2` contain only lowercase letters from the set `{'a', 'b', 'c', 'd', 'e', 'f'}`.
* `s2` is an anagram of `s1`.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    @cache
    def kSimilarity(self, s1: str, s2: str) -> int:
        chars1, chars2 = [], []
        ret = len(s1) - 1

        for i in range(len(s1)):
            if s1[i] != s2[i]:
                chars1.append(s1[i])
                chars2.append(s2[i])

        ret = max(len(chars1) - 1, 0)

        for i in range(1, len(chars1)):
            if chars1[i] == chars2[0]:
                chars1[i] = chars1[0]
                ret = min(
                    ret, 1 + self.kSimilarity(''.join(chars1[1:]), ''.join(chars2[1:])))
                chars1[i] = chars2[0]

        return ret
```
