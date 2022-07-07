# 2047. 句子中的有效单词数
句子仅由小写字母（`'a'` 到 `'z'`）、数字（`'0'` 到 `'9'`）、连字符（`'-'`）、标点符号（`'!'`、`'.'` 和 `','`）以及空格（`' '`）组成。每个句子可以根据空格分解成 **一个或者多个 token** ，这些 token 之间由一个或者多个空格 `' '` 分隔。

如果一个 token 同时满足下述条件，则认为这个 token 是一个有效单词：
* 仅由小写字母、连字符和/或标点（不含数字）组成。
* **至多一个** 连字符 `'-'` 。如果存在，连字符两侧应当都存在小写字母（`"a-b"` 是一个有效单词，但 `"-ab"` 和 `"ab-"` 不是有效单词）。
* **至多一个** 标点符号。如果存在，标点符号应当位于 token 的 **末尾** 。

这里给出几个有效单词的例子：`"a-b."`、`"afad"`、`"ba-c"`、`"a!"` 和 `"!"` 。

给你一个字符串 `sentence` ，请你找出并返回 `sentence` 中 **有效单词的数目** 。

#### 示例 1:
<pre>
<strong>输入:</strong> sentence = "cat and  dog"
<strong>输出:</strong> 3
<strong>解释:</strong> 句子中的有效单词是 "cat"、"and" 和 "dog"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> sentence = "!this  1-s b8d!"
<strong>输出:</strong> 0
<strong>解释:</strong> 句子中没有有效单词
"!this" 不是有效单词，因为它以一个标点开头
"1-s" 和 "b8d" 也不是有效单词，因为它们都包含数字
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> sentence = "alice and  bob are playing stone-game10"
<strong>输出:</strong> 5
<strong>解释:</strong> 句子中的有效单词是 "alice"、"and"、"bob"、"are" 和 "playing"
"stone-game10" 不是有效单词，因为它含有数字
</pre>

#### 提示:
* `1 <= sentence.length <= 1000`
* `sentence` 由小写英文字母、数字（`0-9`）、以及字符（`' '`、`'-'`、`'!'`、`'.'` 和 `','`）组成
* 句子中至少有 `1` 个 token

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def countValidWords(self, sentence: str) -> int:
        return sum(1 for word in sentence.split() if self.is_valid(word))

    def is_valid(self, word: str) -> bool:
        no_hyphen = True

        for i in range(len(word)):
            if word[i].isdigit():
                return False
            elif word[i] == '-':
                if not no_hyphen:
                    return False
                elif i == 0 or i == len(word) - 1:
                    return False
                elif not (word[i - 1].islower() and word[i + 1].islower()):
                    return False
                else:
                    no_hyphen = False
            elif word[i] in "!.," and i != len(word) - 1:
                return False

        return True
```
