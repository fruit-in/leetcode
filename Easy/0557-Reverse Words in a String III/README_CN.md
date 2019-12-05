# 557. 反转字符串中的单词 III
给定一个字符串，你需要反转字符串中每个单词的字符顺序，同时仍保留空格和单词的初始顺序。

#### 示例 1:
<pre>
<strong>输入:</strong> "Let's take LeetCode contest"
<strong>输出:</strong> "s'teL ekat edoCteeL tsetnoc"
</pre>

**Note:** 在字符串中，每个单词由单个空格分隔，并且字符串中不会有任何额外的空格。

## 题解 (Python)

### 1. 题解
```Python3
class Solution:
    def reverseWords(self, s: str) -> str:
        return ' '.join(word[::-1] for word in s.split())
```
