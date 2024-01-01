# 1657. 确定两个字符串是否接近
如果可以使用以下操作从一个字符串得到另一个字符串，则认为两个字符串 **接近** ：

* 操作 1：交换任意两个 **现有** 字符。
    * 例如，`abcde -> aecdb`
* 操作 2：将一个 **现有** 字符的每次出现转换为另一个 **现有** 字符，并对另一个字符执行相同的操作。
    * 例如，`aacabb -> bbcbaa`（所有 `a` 转化为 `b` ，而所有的 `b` 转换为 `a` ）

你可以根据需要对任意一个字符串多次使用这两种操作。

给你两个字符串，`word1` 和 `word2` 。如果 `word1` 和 `word2` **接近** ，就返回 `true` ；否则，返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> word1 = "abc", word2 = "bca"
<strong>输出:</strong> true
<strong>解释:</strong> 2 次操作从 word1 获得 word2 。
执行操作 1："abc" -> "acb"
执行操作 1："acb" -> "bca"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> word1 = "a", word2 = "aa"
<strong>输出:</strong> false
<strong>解释:</strong> 不管执行多少次操作，都无法从 word1 得到 word2 ，反之亦然。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> word1 = "cabbba", word2 = "abbccc"
<strong>输出:</strong> true
<strong>解释:</strong> 3 次操作从 word1 获得 word2 。
执行操作 1："cabbba" -> "caabbb"
执行操作 2："caabbb" -> "baaccc"
执行操作 2："baaccc" -> "abbccc"
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> word1 = "cabbba", word2 = "aabbss"
<strong>输出:</strong> false
<strong>解释:</strong> 不管执行多少次操作，都无法从 word1 得到 word2 ，反之亦然。
</pre>

#### 提示:
* <code>1 <= word1.length, word2.length <= 10<sup>5</sup></code>
* `word1` 和 `word2` 仅包含小写英文字母

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def closeStrings(self, word1: str, word2: str) -> bool:
        if len(word1) != len(word2) or set(word1) != set(word2):
            return False

        count1 = [0] * 26
        count2 = [0] * 26

        for ch1, ch2 in zip(word1, word2):
            count1[ord(ch1) - 97] += 1
            count2[ord(ch2) - 97] += 1

        return sorted(count1) == sorted(count2)
```
