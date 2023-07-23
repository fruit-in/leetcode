# 1592. 重新排列单词间的空格
给你一个字符串 `text` ，该字符串由若干被空格包围的单词组成。每个单词由一个或者多个小写英文字母组成，并且两个单词之间至少存在一个空格。题目测试用例保证 `text` **至少包含一个单词** 。

请你重新排列空格，使每对相邻单词之间的空格数目都 **相等** ，并尽可能 **最大化** 该数目。如果不能重新平均分配所有空格，请 **将多余的空格放置在字符串末尾** ，这也意味着返回的字符串应当与原 `text` 字符串的长度相等。

返回 **重新排列空格后的字符串** 。

#### 示例 1:
<pre>
<b>输入:</b> text = "  this   is  a sentence "
<b>输出:</b> "this   is   a   sentence"
<b>解释:</b> 总共有 9 个空格和 4 个单词。可以将 9 个空格平均分配到相邻单词之间，相邻单词间空格数为：9 / (4-1) = 3 个。
</pre>

#### 示例 2:
<pre>
<b>输入:</b> text = " practice   makes   perfect"
<b>输出:</b> "practice   makes   perfect "
<b>解释:</b> 总共有 7 个空格和 3 个单词。7 / (3-1) = 3 个空格加上 1 个多余的空格。多余的空格需要放在字符串的末尾。
</pre>

#### 示例 3:
<pre>
<b>输入:</b> text = "hello   world"
<b>输出:</b> "hello   world"
</pre>

#### 示例 4:
<pre>
<b>输入:</b> text = "  walks  udp package   into  bar a"
<b>输出:</b> "walks  udp  package  into  bar  a "
</pre>

#### 示例 5:
<pre>
<b>输入:</b> text = "a"
<b>输出:</b> "a"
</pre>

#### 提示:
* `1 <= text.length <= 100`
* `text` 由小写英文字母和 `' '` 组成
* `text` 中至少包含一个单词

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def reorderSpaces(self, text: str) -> str:
        words = list(filter(lambda x: x != '', text.split(' ')))
        if len(words) == 1:
            div, mod = 0, text.count(' ')
        else:
            div, mod = divmod(text.count(' '), len(words) - 1)

        return (div * ' ').join(words) + mod * ' '
```
