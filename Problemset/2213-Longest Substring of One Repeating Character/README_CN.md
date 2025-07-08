# 2213. 由单个字符重复的最长子字符串
给你一个下标从 **0** 开始的字符串 `s` 。另给你一个下标从 **0** 开始、长度为 `k` 的字符串 `queryCharacters` ，一个下标从 `0` 开始、长度也是 `k` 的整数 **下标** 数组 `queryIndices` ，这两个都用来描述 `k` 个查询。

第 `i` 个查询会将 `s` 中位于下标 `queryIndices[i]` 的字符更新为 `queryCharacters[i]` 。

返回一个长度为 `k` 的数组 `lengths` ，其中 `lengths[i]` 是在执行第 `i` 个查询 **之后** `s` 中仅由 **单个字符重复** 组成的 **最长子字符串** 的 **长度** 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "babacc", queryCharacters = "bcb", queryIndices = [1,3,3]
<strong>输出:</strong> [3,3,4]
<strong>解释:</strong>
- 第 1 次查询更新后 s = "bbbacc" 。由单个字符重复组成的最长子字符串是 "bbb" ，长度为 3 。
- 第 2 次查询更新后 s = "bbbccc" 。由单个字符重复组成的最长子字符串是 "bbb" 或 "ccc"，长度为 3 。
- 第 3 次查询更新后 s = "bbbbcc" 。由单个字符重复组成的最长子字符串是 "bbbb" ，长度为 4 。
因此，返回 [3,3,4] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "abyzz", queryCharacters = "aa", queryIndices = [2,1]
<strong>输出:</strong> [2,3]
<strong>解释:</strong>
- 第 1 次查询更新后 s = "abazz" 。由单个字符重复组成的最长子字符串是 "zz" ，长度为 2 。
- 第 2 次查询更新后 s = "aaazz" 。由单个字符重复组成的最长子字符串是 "aaa" ，长度为 3 。
因此，返回 [2,3] 。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` 由小写英文字母组成
* `k == queryCharacters.length == queryIndices.length`
* <code>1 <= k <= 10<sup>5</sup></code>
* `queryCharacters` 由小写英文字母组成
* `0 <= queryIndices[i] < s.length`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def longestRepeating(self, s: str, queryCharacters: str, queryIndices: List[int]) -> List[int]:
        size = 1 << ceil(log2(len(s)))
        treelength = [0] * (2 * size)
        treeleft = [('', 0)] * (2 * size)
        treeright = [('', 0)] * (2 * size)
        treerepeat = [0] * (2 * size)
        lengths = []

        for i, c in enumerate(s):
            treelength[size + i] = 1
            treeleft[size + i] = (c, 1)
            treeright[size + i] = (c, 1)
            treerepeat[size + i] = 1
        for i in range(size - 1, 0, -1):
            treelength[i] = treelength[2 * i] + treelength[2 * i + 1]
            treeleft[i] = treeleft[2 * i]
            treeright[i] = treeright[2 * i + 1]
            treerepeat[i] = max(treerepeat[2 * i], treerepeat[2 * i + 1])
            if treeleft[2 * i][1] == treelength[2 * i] and treeleft[2 * i][0] == treeleft[2 * i + 1][0]:
                treeleft[i] = (treeleft[i][0], treeleft[i]
                               [1] + treeleft[2 * i + 1][1])
            if treeright[2 * i + 1][1] == treelength[2 * i + 1] and treeright[2 * i + 1][0] == treeright[2 * i][0]:
                treeright[i] = (treeright[i][0], treeright[i]
                                [1] + treeright[2 * i][1])
            if treeright[2 * i][0] == treeleft[2 * i + 1][0]:
                treerepeat[i] = max(
                    treerepeat[i], treeright[2 * i][1] + treeleft[2 * i + 1][1])

        for c, i in zip(queryCharacters, queryIndices):
            i += size
            treeleft[i] = (c, 1)
            treeright[i] = (c, 1)
            while i > 1:
                i >>= 1
                treeleft[i] = treeleft[2 * i]
                treeright[i] = treeright[2 * i + 1]
                treerepeat[i] = max(treerepeat[2 * i], treerepeat[2 * i + 1])
                if treeleft[2 * i][1] == treelength[2 * i] and treeleft[2 * i][0] == treeleft[2 * i + 1][0]:
                    treeleft[i] = (treeleft[i][0], treeleft[i]
                                   [1] + treeleft[2 * i + 1][1])
                if treeright[2 * i + 1][1] == treelength[2 * i + 1] and treeright[2 * i + 1][0] == treeright[2 * i][0]:
                    treeright[i] = (treeright[i][0], treeright[i]
                                    [1] + treeright[2 * i][1])
                if treeright[2 * i][0] == treeleft[2 * i + 1][0]:
                    treerepeat[i] = max(
                        treerepeat[i], treeright[2 * i][1] + treeleft[2 * i + 1][1])

            lengths.append(treerepeat[1])

        return lengths
```
