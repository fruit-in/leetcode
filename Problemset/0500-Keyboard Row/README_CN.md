# 500. 键盘行
给定一个单词列表，只返回可以使用在键盘同一行的字母打印出来的单词。键盘如下图所示。

![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/12/keyboard.png)

#### 示例:
<pre>
<strong>输入:</strong> ["Hello", "Alaska", "Dad", "Peace"]
<strong>输出:</strong> ["Alaska", "Dad"]
</pre>

#### 注意:
1. 你可以重复使用键盘上同一字符。
2. 你可以假设输入的字符串将只包含字母。

## 题解 (Python)

### 1. 集合
```Python3
class Solution:
    def findWords(self, words: List[str]) -> List[str]:
        result = []
        row_q = set("qwertyuiop")
        row_a = set("asdfghjkl")
        row_z = set("zxcvbnm")
        for word in words:
            word_set = set(word.lower())
            if word_set <= row_q or word_set <= row_a or word_set <= row_z:
                result.append(word)
        return result
```
