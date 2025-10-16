# 960. Delete Columns to Make Sorted III
You are given an array of `n` strings `strs`, all of the same length.

We may choose any deletion indices, and we delete all the characters in those indices for each string.

For example, if we have `strs = ["abcdef","uvwxyz"]` and deletion indices `{0, 2, 3}`, then the final array after deletions is `["bef", "vyz"]`.

Suppose we chose a set of deletion indices `answer` such that after deletions, the final array has **every string (row) in lexicographic** order. (i.e., `(strs[0][0] <= strs[0][1] <= ... <= strs[0][strs[0].length - 1])`, and `(strs[1][0] <= strs[1][1] <= ... <= strs[1][strs[1].length - 1])`, and so on). Return *the minimum possible value of* `answer.length`.

#### Example 1:
<pre>
<strong>Input:</strong> strs = ["babca","bbazb"]
<strong>Output:</strong> 3
<strong>Explanation:</strong> After deleting columns 0, 1, and 4, the final array is strs = ["bc", "az"].
Both these rows are individually in lexicographic order (ie. strs[0][0] <= strs[0][1] and strs[1][0] <= strs[1][1]).
Note that strs[0] > strs[1] - the array strs is not necessarily in lexicographic order.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> strs = ["edcba"]
<strong>Output:</strong> 4
<strong>Explanation:</strong> If we delete less than 4 columns, the only row will not be lexicographically sorted.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> strs = ["ghi","def","abc"]
<strong>Output:</strong> 0
<strong>Explanation:</strong> All rows are already lexicographically sorted.
</pre>

#### Constraints:
* `n == strs.length`
* `1 <= n <= 100`
* `1 <= strs[i].length <= 100`
* `strs[i]` consists of lowercase English letters.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def minDeletionSize(self, strs: List[str]) -> int:
        m = len(strs[0])
        dp = [[0, i] for i in range(m)]
        dp[0][0] = 1

        for i in range(1, m):
            dp[i][0] = min(dp[i - 1]) + 1
            for j in range(i):
                if all(s[j] <= s[i] for s in strs):
                    dp[i][1] = min(dp[i][1], dp[j][1] + i - j - 1)

        return min(dp[-1])
```
