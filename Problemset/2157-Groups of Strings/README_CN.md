# 2157. 字符串分组
给你一个下标从 **0** 开始的字符串数组 `words` 。每个字符串都只包含 **小写英文字母** 。`words` 中任意一个子串中，每个字母都至多只出现一次。

如果通过以下操作之一，我们可以从 `s1` 的字母集合得到 `s2` 的字母集合，那么我们称这两个字符串为 **关联的** ：
* 往 `s1` 的字母集合中添加一个字母。
* 从 `s1` 的字母集合中删去一个字母。
* 将 `s1` 中的一个字母替换成另外任意一个字母（也可以替换为这个字母本身）。

数组 `words` 可以分为一个或者多个无交集的 **组** 。如果一个字符串与另一个字符串关联，那么它们应当属于同一个组。

注意，你需要确保分好组后，一个组内的任一字符串与其他组的字符串都不关联。可以证明在这个条件下，分组方案是唯一的。

请你返回一个长度为 `2` 的数组 `ans` ：
* `ans[0]` 是 `words` 分组后的 **总组数** 。
* `ans[1]` 是字符串数目最多的组所包含的字符串数目。

#### 示例 1:
<pre>
<strong>输入:</strong> words = ["a","b","ab","cde"]
<strong>输出:</strong> [2,3]
<strong>解释:</strong>
- words[0] 可以得到 words[1] （将 'a' 替换为 'b'）和 words[2] （添加 'b'）。所以 words[0] 与 words[1] 和 words[2] 关联。
- words[1] 可以得到 words[0] （将 'b' 替换为 'a'）和 words[2] （添加 'a'）。所以 words[1] 与 words[0] 和 words[2] 关联。
- words[2] 可以得到 words[0] （删去 'b'）和 words[1] （删去 'a'）。所以 words[2] 与 words[0] 和 words[1] 关联。
- words[3] 与 words 中其他字符串都不关联。
所以，words 可以分成 2 个组 ["a","b","ab"] 和 ["cde"] 。最大的组大小为 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> words = ["a","ab","abc"]
<strong>输出:</strong> [1,3]
<strong>解释:</strong>
- words[0] 与 words[1] 关联。
- words[1] 与 words[0] 和 words[2] 关联。
- words[2] 与 words[1] 关联。
由于所有字符串与其他字符串都关联，所以它们全部在同一个组内。
所以最大的组大小为 3 。
</pre>

#### 提示:
* <code>1 <= words.length <= 2 * 10<sup>4</sup></code>
* `1 <= words[i].length <= 26`
* `words[i]` 只包含小写英文字母。
* `words[i]` 中每个字母最多只出现一次。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def groupStrings(self, words: List[str]) -> List[int]:
        counter = {}
        parent = {}
        groups = {}
        largestsize = 0

        for s1 in sorted(words, key=len):
            mask1 = reduce(lambda x, y: x | (1 << (ord(y) - 97)), s1, 0)
            if mask1 in counter:
                counter[mask1] += 1
                continue
            counter[mask1] = 1
            parent[mask1] = mask1

            for i in range(26):
                if (mask1 >> i) & 1 == 0:
                    continue

                mask2 = mask1 ^ (1 << i)
                if mask2 in counter:
                    while parent[mask2] != parent[parent[mask2]]:
                        parent[mask2] = parent[parent[mask2]]
                    parent[parent[mask2]] = mask1

                for j in range(26):
                    if (mask2 >> j) & 1 == 1:
                        continue

                    mask3 = mask2 ^ (1 << j)
                    if mask3 in counter:
                        while parent[mask3] != parent[parent[mask3]]:
                            parent[mask3] = parent[parent[mask3]]
                        parent[parent[mask3]] = mask1

        for mask, count in counter.items():
            while parent[mask] != parent[parent[mask]]:
                parent[mask] = parent[parent[mask]]
            groups[parent[mask]] = groups.get(parent[mask], 0) + count
            largestsize = max(largestsize, groups[parent[mask]])

        return [len(groups), largestsize]
```
