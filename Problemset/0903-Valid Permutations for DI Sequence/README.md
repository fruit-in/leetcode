# 903. Valid Permutations for DI Sequence
You are given a string `s` of length `n` where `s[i]` is either:
* `'D'` means decreasing, or
* `'I'` means increasing.

A permutation `perm` of `n + 1` integers of all the integers in the range `[0, n]` is called a **valid permutation** if for all valid `i`:
* If `s[i] == 'D'`, then `perm[i] > perm[i + 1]`, and
* If `s[i] == 'I'`, then `perm[i] < perm[i + 1]`.

Return *the number of **valid permutations*** `perm`. Since the answer may be large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> s = "DID"
<strong>Output:</strong> 5
<strong>Explanation:</strong> The 5 valid permutations of (0, 1, 2, 3) are:
(1, 0, 3, 2)
(2, 0, 3, 1)
(2, 1, 3, 0)
(3, 0, 2, 1)
(3, 1, 2, 0)
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "D"
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `n == s.length`
* `1 <= n <= 200`
* `s[i]` is either `'I'` or `'D'`.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def numPermsDISequence(self, s: str) -> int:
        @cache
        def dfs(i: int, greater: int) -> int:
            if i == len(s):
                return 1

            less = len(s) - i - greater
            ret = 0

            if s[i] == 'D':
                for x in range(less):
                    ret = (ret + dfs(i + 1, less - 1 - x + greater)) % 1000000007
            else:
                for x in range(greater):
                    ret = (ret + dfs(i + 1, greater - 1 - x)) % 1000000007

            return ret

        s = "I" + s

        return dfs(0, len(s))
```
