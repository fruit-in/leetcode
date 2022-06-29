# 2000. 反转单词前缀
给你一个下标从 **0** 开始的字符串 `word` 和一个字符 `ch` 。找出 `ch` 第一次出现的下标 `i` ，**反转** `word` 中从下标 `0` 开始、直到下标 `i` 结束（含下标 `i` ）的那段字符。如果 `word` 中不存在字符 `ch` ，则无需进行任何操作。
* 例如，如果 `word = "abcdefd"` 且 `ch = "d"` ，那么你应该 **反转** 从下标 0 开始、直到下标 `3` 结束（含下标 `3` ）。结果字符串将会是 `"dcbaefd"` 。

返回 **结果字符串** 。

#### 示例 1:
<pre>
<strong>输入:</strong> word = "abcdefd", ch = "d"
<strong>输出:</strong> "dcbaefd"
<strong>解释:</strong> "d" 第一次出现在下标 3 。
反转从下标 0 到下标 3（含下标 3）的这段字符，结果字符串是 "dcbaefd" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> word = "xyxzxe", ch = "z"
<strong>输出:</strong> "zxyxxe"
<strong>解释:</strong> "z" 第一次也是唯一一次出现是在下标 3 。
反转从下标 0 到下标 3（含下标 3）的这段字符，结果字符串是 "zxyxxe" 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> word = "abcd", ch = "z"
<strong>输出:</strong> "abcd"
<strong>解释:</strong> "z" 不存在于 word 中。
无需执行反转操作，结果字符串是 "abcd" 。
</pre>

#### 提示:
* `1 <= word.length <= 250`
* `word` 由小写英文字母组成
* `ch` 是一个小写英文字母

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def reversePrefix(self, word: str, ch: str) -> str:
        i = word.find(ch)

        return word[:i + 1][::-1] + word[i + 1:]
```
