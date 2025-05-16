# 2014. 重复 K 次的最长子序列
给你一个长度为 `n` 的字符串 `s` ，和一个整数 `k` 。请你找出字符串 `s` 中 **重复** `k` 次的 **最长子序列** 。

**子序列** 是由其他字符串删除某些（或不删除）字符派生而来的一个字符串。

如果 `seq * k` 是 `s` 的一个子序列，其中 `seq * k` 表示一个由 `seq` 串联 `k` 次构造的字符串，那么就称 `seq` 是字符串 `s` 中一个 **重复** `k` 次 的子序列。

* 举个例子，`"bba"` 是字符串 `"bababcba"` 中的一个重复 `2` 次的子序列，因为字符串 `"bbabba"` 是由 `"bba"` 串联 `2` 次构造的，而 `"bbabba"` 是字符串 `"bababcba"` 的一个子序列。

返回字符串 `s` 中 **重复 k 次的最长子序列**  。如果存在多个满足的子序列，则返回 **字典序最大** 的那个。如果不存在这样的子序列，返回一个 **空** 字符串。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/08/30/longest-subsequence-repeat-k-times.png)
<pre>
<strong>输入:</strong> s = "letsleetcode", k = 2
<strong>输出:</strong> "let"
<strong>解释:</strong> 存在两个最长子序列重复 2 次：let" 和 "ete" 。
"let" 是其中字典序最大的一个。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "bb", k = 2
<strong>输出:</strong> "b"
<strong>解释:</strong> 重复 2 次的最长子序列是 "b" 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "ab", k = 2
<strong>输出:</strong> ""
<strong>解释:</strong> 不存在重复 2 次的最长子序列。返回空字符串。
</pre>

#### 提示:
* `n == s.length`
* `2 <= n, k <= 2000`
* `2 <= n < k * 8`
* `s` 由小写英文字母组成

## 题解 (Python)

### 1. 题解
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
