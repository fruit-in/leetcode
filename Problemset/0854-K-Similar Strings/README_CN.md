# 854. 相似度为 K 的字符串
对于某些非负整数 `k` ，如果交换 `s1` 中两个字母的位置恰好 `k` 次，能够使结果字符串等于 `s2` ，则认为字符串 `s1` 和 `s2` 的 **相似度为** `k` 。

给你两个字母异位词 `s1` 和 `s2` ，返回 `s1` 和 `s2` 的相似度 `k` 的最小值。

#### 示例 1:
<pre>
<strong>输入:</strong> s1 = "ab", s2 = "ba"
<strong>输出:</strong> 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s1 = "abc", s2 = "bca"
<strong>输出:</strong> 2
</pre>

#### 提示:
* `1 <= s1.length <= 20`
* `s2.length == s1.length`
* `s1` 和 `s2`  只包含集合 `{'a', 'b', 'c', 'd', 'e', 'f'}` 中的小写字母
* `s2` 是 `s1` 的一个字母异位词

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    @cache
    def kSimilarity(self, s1: str, s2: str) -> int:
        chars1, chars2 = [], []
        ret = len(s1) - 1

        for i in range(len(s1)):
            if s1[i] != s2[i]:
                chars1.append(s1[i])
                chars2.append(s2[i])

        ret = max(len(chars1) - 1, 0)

        for i in range(1, len(chars1)):
            if chars1[i] == chars2[0]:
                chars1[i] = chars1[0]
                ret = min(
                    ret, 1 + self.kSimilarity(''.join(chars1[1:]), ''.join(chars2[1:])))
                chars1[i] = chars2[0]

        return ret
```
