# 1147. Longest Chunked Palindrome Decomposition
You are given a string `text`. You should split it to k substrings <code>(subtext<sub>1</sub>, subtext<sub>2</sub>, ..., subtext<sub>k</sub>)</code> such that:
* <code>subtext<sub>i</sub></code> is a **non-empty** string.
* The concatenation of all the substrings is equal to `text` (i.e., <code>subtext<sub>1</sub> + subtext<sub>2</sub> + ... + subtext<sub>k</sub> == text</code>).
* <code>subtext<sub>i</sub> == subtext<sub>k - i + 1</sub></code> for all valid values of `i` (i.e., `1 <= i <= k`).

Return the largest possible value of `k`.

#### Example 1:
<pre>
<strong>Input:</strong> text = "ghiabcdefhelloadamhelloabcdefghi"
<strong>Output:</strong> 7
<strong>Explanation:</strong> We can split the string on "(ghi)(abcdef)(hello)(adam)(hello)(abcdef)(ghi)".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> text = "merchant"
<strong>Output:</strong> 1
<strong>Explanation:</strong> We can split the string on "(merchant)".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> text = "antaprezatepzapreanta"
<strong>Output:</strong> 11
<strong>Explanation:</strong> We can split the string on "(a)(nt)(a)(pre)(za)(tep)(za)(pre)(a)(nt)(a)".
</pre>

#### Constraints:
* `1 <= text.length <= 1000`
* `text` consists only of lowercase English characters.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def longestDecomposition(self, text: str) -> int:
        @cache
        def dfs(start: int) -> int:
            n = len(text) - start * 2

            if n < 2:
                return n

            j = 0
            lps = [0] * n
            ret = 1

            for i in range(1, n):
                while j > 0 and text[start + i] != text[start + j]:
                    j = lps[j - 1]

                if text[start + i] == text[start + j]:
                    j += 1
                    lps[i] = j

            j = lps[-1] - 1
            while j >= 0:
                if text[start + n - 1] == text[start + j]:
                    ret = max(ret, 2 + dfs(start + j + 1))
                j = lps[j] - 1

            return ret

        return dfs(0)
```
