# 960. 删列造序 III
给定由 `n` 个小写字母字符串组成的数组 `strs` ，其中每个字符串长度相等。

选取一个删除索引序列，对于 `strs` 中的每个字符串，删除对应每个索引处的字符。

比如，有 `strs = ["abcdef","uvwxyz"]` ，删除索引序列 `{0, 2, 3}` ，删除后为 `["bef", "vyz"]` 。

假设，我们选择了一组删除索引 `answer` ，那么在执行删除操作之后，最终得到的数组的行中的 **每个元素** 都是按**字典序**排列的（即 `(strs[0][0] <= strs[0][1] <= ... <= strs[0][strs[0].length - 1])` 和 `(strs[1][0] <= strs[1][1] <= ... <= strs[1][strs[1].length - 1])` ，依此类推）。

请返回 *`answer.length` 的最小可能值* 。

#### 示例 1:
<pre>
<strong>输入:</strong> strs = ["babca","bbazb"]
<strong>输出:</strong> 3
<strong>解释:</strong>
删除 0、1 和 4 这三列后，最终得到的数组是 strs = ["bc", "az"]。
这两行是分别按字典序排列的（即，strs[0][0] <= strs[0][1] 且 strs[1][0] <= strs[1][1]）。
注意，strs[0] > strs[1] —— 数组 strs 不一定是按字典序排列的。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> strs = ["edcba"]
<strong>输出:</strong> 4
<strong>解释:</strong> 如果删除的列少于 4 列，则剩下的行都不会按字典序排列。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> strs = ["ghi","def","abc"]
<strong>输出:</strong> 0
<strong>解释:</strong> 所有行都已按字典序排列。
</pre>

#### 提示:
* `n == strs.length`
* `1 <= n <= 100`
* `1 <= strs[i].length <= 100`
* `strs[i]` 由小写英文字母组成

## 题解 (Python)

### 1. 题解
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
