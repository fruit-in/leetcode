# 1935. 可以输入的最大单词数
键盘出现了一些故障，有些字母键无法正常工作。而键盘上所有其他键都能够正常工作。

给你一个由若干单词组成的字符串 `text` ，单词间由单个空格组成（不含前导和尾随空格）；另有一个字符串 `brokenLetters` ，由所有已损坏的不同字母键组成，返回你可以使用此键盘完全输入的 `text` 中单词的数目。

#### 示例 1:
<pre>
<strong>输入:</strong> text = "hello world", brokenLetters = "ad"
<strong>输出:</strong> 1
<strong>解释:</strong> 无法输入 "world" ，因为字母键 'd' 已损坏。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> text = "leet code", brokenLetters = "lt"
<strong>输出:</strong> 1
<strong>解释:</strong> 无法输入 "leet" ，因为字母键 'l' 和 't' 已损坏。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> text = "leet code", brokenLetters = "e"
<strong>输出:</strong> 0
<strong>解释:</strong> 无法输入任何单词，因为字母键 'e' 已损坏。
</pre>

#### 提示:
* <code>1 <= text.length <= 10<sup>4</sup></code>
* `0 <= brokenLetters.length <= 26`
* `text` 由若干用单个空格分隔的单词组成，且不含任何前导和尾随空格
* 每个单词仅由小写英文字母组成
* `brokenLetters` 由 **互不相同** 的小写英文字母组成

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def canBeTypedWords(self, text: str, brokenLetters: str) -> int:
        ret = 0

        for word in text.split():
            if all(c not in brokenLetters for c in word):
                ret += 1

        return ret
```
