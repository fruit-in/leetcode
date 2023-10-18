# 1202. 交换字符串中的元素
给你一个字符串 `s`，以及该字符串中的一些「索引对」数组 `pairs`，其中 `pairs[i] = [a, b]` 表示字符串中的两个索引（编号从 0 开始）。

你可以 **任意多次交换** 在 `pairs` 中任意一对索引处的字符。

返回在经过若干次交换后，`s` 可以变成的按字典序最小的字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "dcab", pairs = [[0,3],[1,2]]
<strong>输出:</strong> "bacd"
<strong>解释:</strong>
交换 s[0] 和 s[3], s = "bcad"
交换 s[1] 和 s[2], s = "bacd"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "dcab", pairs = [[0,3],[1,2],[0,2]]
<strong>输出:</strong> "abcd"
<strong>解释:</strong>
交换 s[0] 和 s[3], s = "bcad"
交换 s[0] 和 s[2], s = "acbd"
交换 s[1] 和 s[2], s = "abcd"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "cba", pairs = [[0,1],[1,2]]
<strong>输出:</strong> "abc"
<strong>解释:</strong>
交换 s[0] 和 s[1], s = "bca"
交换 s[1] 和 s[2], s = "bac"
交换 s[0] 和 s[1], s = "abc"
</pre>

#### Constraints:
* `1 <= s.length <= 10^5`
* `0 <= pairs.length <= 10^5`
* `0 <= pairs[i][0], pairs[i][1] < s.length`
* `s` 中只含有小写英文字母

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def smallestStringWithSwaps(self, s: str, pairs: List[List[int]]) -> str:
        parent = list(range(len(s)))
        groups = {}
        chars = []

        for a, b in pairs:
            while parent[a] != parent[parent[a]]:
                parent[a] = parent[parent[a]]
            while parent[b] != parent[parent[b]]:
                parent[b] = parent[parent[b]]

            if parent[a] < parent[b]:
                parent[parent[b]] = parent[a]
            else:
                parent[parent[a]] = parent[b]

        for i in range(len(s)):
            while parent[i] != parent[parent[i]]:
                parent[i] = parent[parent[i]]

            if parent[i] not in groups:
                groups[parent[i]] = []
            groups[parent[i]].append(i)

        for group in groups:
            groups[group].sort(key=lambda i: -ord(s[i]))

        for i in range(len(s)):
            chars.append(s[groups[parent[i]].pop()])

        return ''.join(chars)
```
