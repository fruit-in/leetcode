# 943. 最短超级串
给定一个字符串数组 `words`，找到以 `words` 中每个字符串作为子字符串的最短字符串。如果有多个有效最短字符串满足题目条件，返回其中 **任意一个** 即可。

我们可以假设 `words` 中没有字符串是 `words` 中另一个字符串的子字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> words = ["alex","loves","leetcode"]
<strong>输出:</strong> "alexlovesleetcode"
<strong>解释:</strong> "alex"，"loves"，"leetcode" 的所有排列都会被接受。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> words = ["catg","ctaagt","gcta","ttca","atgcatc"]
<strong>输出:</strong> "gctaagttcatgcatc"
</pre>

#### 提示:
* `1 <= words.length <= 12`
* `1 <= words[i].length <= 20`
* `words[i]` 由小写英文字母组成
* `words` 中的所有字符串 **互不相同**

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def shortestSuperstring(self, words: List[str]) -> str:
        @cache
        def shortestLength(mask: int, prev: int) -> int:
            if mask == (1 << n) - 1:
                return len(words[prev])

            ret = inf

            for i in range(n):
                if (mask >> i) & 1 == 0:
                    length = shortestLength(
                        mask | (1 << i), i) - overlap[(prev, i)]
                    ret = min(ret, len(words[prev]) + length)

            return ret

        n = len(words)
        overlap = {(n, i): 0 for i in range(n)}
        mask = 0
        prev = n
        parts = []

        for i in range(n):
            for j in range(i + 1, n):
                overlap[(i, j)] = 0
                overlap[(j, i)] = 0
                for k in range(1, min(len(words[i]), len(words[j]))):
                    if words[i][-k:] == words[j][:k]:
                        overlap[(i, j)] = k
                    if words[j][-k:] == words[i][:k]:
                        overlap[(j, i)] = k

        for _ in range(n):
            minlength = inf
            minlengthi = n

            for i in range(n):
                if (mask >> i) & 1 == 0:
                    length = shortestLength(
                        mask | (1 << i), i) - overlap[(prev, i)]
                    if length < minlength:
                        minlength = length
                        minlengthi = i

            parts.append(words[minlengthi][overlap[(prev, minlengthi)]:])
            mask |= 1 << minlengthi
            prev = minlengthi

        return ''.join(parts)
```
