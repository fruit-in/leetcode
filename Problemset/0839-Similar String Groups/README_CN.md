# 839. 相似字符串组
如果交换字符串 `X` 中的两个不同位置的字母，使得它和字符串 `Y` 相等，那么称 `X` 和 `Y` 两个字符串相似。如果这两个字符串本身是相等的，那它们也是相似的。

例如，`"tars"` 和 `"rats"` 是相似的 (交换 `0` 与 `2` 的位置)； `"rats"` 和 `"arts"` 也是相似的，但是 `"star"` 不与 `"tars"`，`"rats"`，或 `"arts"` 相似。

总之，它们通过相似性形成了两个关联组：`{"tars", "rats", "arts"}` 和 `{"star"}`。注意，`"tars"` 和 `"arts"` 是在同一组中，即使它们并不相似。形式上，对每个组而言，要确定一个单词在组中，只需要这个词和该组中至少一个单词相似。

给你一个字符串列表 `strs`。列表中的每个字符串都是 `strs` 中其它所有字符串的一个字母异位词。请问 `strs` 中有多少个相似字符串组？

#### 示例 1:
<pre>
<strong>输入:</strong> strs = ["tars","rats","arts","star"]
<strong>输出:</strong> 2
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> strs = ["omv","ovm"]
<strong>输出:</strong> 1
</pre>

#### 提示:
* `1 <= strs.length <= 300`
* `1 <= strs[i].length <= 300`
* `strs[i]` 只包含小写字母。
* `strs` 中的所有单词都具有相同的长度，且是彼此的字母异位词。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def numSimilarGroups(self, strs: List[str]) -> int:
        m = len(strs)
        n = len(strs[0])
        parent = list(range(m))

        for i in range(m):
            for j in range(i + 1, m):
                count = 0

                for k in range(n):
                    if strs[i][k] != strs[j][k]:
                        count += 1

                if count <= 2:
                    while parent[i] != parent[parent[i]]:
                        parent[i] = parent[parent[i]]
                    while parent[j] != parent[parent[j]]:
                        parent[j] = parent[parent[j]]
                    if parent[i] < parent[j]:
                        parent[parent[j]] = parent[i]
                    else:
                        parent[parent[i]] = parent[j]

        for i in range(m):
            while parent[i] != parent[parent[i]]:
                parent[i] = parent[parent[i]]

        return sum(1 for i in range(m) if parent[i] == i)
```
