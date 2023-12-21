# 1061. 按字典序排列最小的等效字符串
给出长度相同的两个字符串`s1` 和 `s2` ，还有一个字符串 `baseStr` 。

其中  `s1[i]` 和 `s2[i]`  是一组等价字符。

* 举个例子，如果 `s1 = "abc"` 且 `s2 = "cde"`，那么就有 `'a' == 'c'`, `'b' == 'd'`, `'c' == 'e'`。

等价字符遵循任何等价关系的一般规则：

* **自反性** ：`'a' == 'a'`
* **对称性** ：`'a' == 'b'` 则必定有 `'b' == 'a'`
* **传递性** ：`'a' == 'b'` 且 `'b' == 'c'` 就表明 `'a' == 'c'`

例如， `s1 = "abc"` 和 `s2 = "cde"` 的等价信息和之前的例子一样，那么 `baseStr = "eed"` , `"acd"` 或 `"aab"`，这三个字符串都是等价的，而 `"aab"` 是 `baseStr` 的按字典序最小的等价字符串

利用 `s1` 和 `s2` 的等价信息，找出并返回 `baseStr` 的按字典序排列最小的等价字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> s1 = "parker", s2 = "morris", baseStr = "parser"
<strong>输出:</strong> "makkek"
<strong>解释:</strong> 根据 A 和 B 中的等价信息，我们可以将这些字符分为 [m,p], [a,o], [k,r,s], [e,i] 共 4 组。每组中的字符都是等价的，并按字典序排列。所以答案是 "makkek"。
So the answer is "makkek".
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s1 = "hello", s2 = "world", baseStr = "hold"
<strong>输出:</strong> "hdld"
<strong>解释:</strong> 根据 A 和 B 中的等价信息，我们可以将这些字符分为 [h,w], [d,e,o], [l,r] 共 3 组。所以只有 S 中的第二个字符 'o' 变成 'd'，最后答案为 "hdld"。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s1 = "leetcode", s2 = "programs", baseStr = "sourcecode"
<strong>输出:</strong> "aauaaaaada"
<strong>解释:</strong> 我们可以把 A 和 B 中的等价字符分为 [a,o,e,r,s,c], [l,p], [g,t] 和 [d,m] 共 4 组，因此 S 中除了 'u' 和 'd' 之外的所有字母都转化成了 'a'，最后答案为 "aauaaaaada"。
</pre>

#### 提示:
* `1 <= s1.length, s2.length, baseStr <= 1000`
* `s1.length == s2.length`
* 字符串`s1`, `s2`, and `baseStr` 仅由从 `'a'` 到 `'z'` 的小写英文字母组成。
* `s1`, `s2`, and `baseStr` consist of lowercase English letters.

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def smallestEquivalentString(self, s1: str, s2: str, baseStr: str) -> str:
        equivalent = list(range(26))

        for ch1, ch2 in zip(s1, s2):
            ch1, ch2 = ord(ch1) - 97, ord(ch2) - 97

            while equivalent[equivalent[ch1]] != equivalent[ch1]:
                equivalent[ch1] = equivalent[equivalent[ch1]]
            while equivalent[equivalent[ch2]] != equivalent[ch2]:
                equivalent[ch2] = equivalent[equivalent[ch2]]

            if equivalent[ch1] < equivalent[ch2]:
                equivalent[equivalent[ch2]] = equivalent[ch1]
            else:
                equivalent[equivalent[ch1]] = equivalent[ch2]

        for ch in range(26):
            while equivalent[equivalent[ch]] != equivalent[ch]:
                equivalent[ch] = equivalent[equivalent[ch]]

        return ''.join(chr(equivalent[ord(ch) - 97] + 97) for ch in baseStr)
```
