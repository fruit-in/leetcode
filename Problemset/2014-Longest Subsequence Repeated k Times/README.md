# 2014. Longest Subsequence Repeated k Times
You are given a string `s` of length `n`, and an integer `k`. You are tasked to find the **longest subsequence repeated** `k` times in string `s`.

A **subsequence** is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.

A subsequence `seq` is **repeated** `k` times in the string `s` if `seq * k` is a subsequence of `s`, where `seq * k` represents a string constructed by concatenating `seq` `k` times.

* For example, `"bba"` is repeated `2` times in the string `"bababcba"`, because the string `"bbabba"`, constructed by concatenating `"bba"` `2` times, is a subsequence of the string `"bababcba"`.

Return *the **longest subsequence repeated*** `k` *times in string* `s`. *If multiple such subsequences are found, return the **lexicographically largest** one. If there is no such subsequence, return an **empty** string*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/08/30/longest-subsequence-repeat-k-times.png)
<pre>
<strong>Input:</strong> s = "letsleetcode", k = 2
<strong>Output:</strong> "let"
<strong>Explanation:</strong> There are two longest subsequences repeated 2 times: "let" and "ete".
"let" is the lexicographically largest one.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "bb", k = 2
<strong>Output:</strong> "b"
<strong>Explanation:</strong> The longest subsequence repeated 2 times is "b".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "ab", k = 2
<strong>Output:</strong> ""
<strong>Explanation:</strong> There is no subsequence repeated 2 times. Empty string is returned.
</pre>

#### Constraints:
* `n == s.length`
* `2 <= n, k <= 2000`
* `2 <= n < k * 8`
* `s` consists of lowercase English letters.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def longestSubsequenceRepeatedK(self, s: str, k: int) -> str:
        def dfs() -> None:
            for ch in count:
                if count[ch] > 0:
                    seq.append(ch)
                    count[ch] -= 1
                    seqs.append(''.join(seq))
                    dfs()
                    seq.pop()
                    count[ch] += 1

        count = {ch: cnt // k for ch,
                 cnt in collections.Counter(s).items() if cnt >= k}
        seq = []
        seqs = []

        dfs()
        seqs.sort(key=lambda seq: (len(seq), seq), reverse=True)

        for seq in seqs:
            i = 0
            j = 0

            for c in s:
                if seq[i] == c:
                    i += 1
                if i == len(seq):
                    i = 0
                    j += 1
                if j == k:
                    return seq

        return ""
```
