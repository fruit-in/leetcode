# 720. 词典中最长的单词
给出一个字符串数组```words```组成的一本英语词典。从中找出最长的一个单词，该单词是由```words```词典中其他单词逐步添加一个字母组成。若其中有多个可行的答案，则返回答案中字典序最小的单词。

若无答案，则返回空字符串。

#### 示例 1:
<pre>
<strong>输入:</strong>
words = ["w","wo","wor","worl", "world"]
<strong>输出:</strong> "world"
<strong>解释:</strong>
单词"world"可由"w", "wo", "wor", 和 "worl"添加一个字母组成。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong>
words = ["a", "banana", "app", "appl", "ap", "apply", "apple"]
<strong>输出:</strong> "apple"
<strong>解释:</strong>
"apply"和"apple"都能由词典中的单词组成。但是"apple"得字典序小于"apply"。
</pre>

#### 注意:
* 所有输入的字符串都只包含小写字母。
* ```words```数组长度范围为```[1,1000]```。
* ```words[i]```的长度范围为```[1,30]```。

## 题解 (Python)

### 1. 暴力法
```Python3
class Solution:
    def longestWord(self, words: List[str]) -> str:
        words_set = set(words)
        ret = ""

        for word in words:
            temp = word
            while temp[:-1] in words_set:
                temp = temp[:-1]

            if not temp[:-1]:
                if len(word) > len(ret):
                    ret = word
                elif len(word) == len(ret) and word < ret:
                    ret = word

        return ret
```
