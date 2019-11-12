# 434. 字符串中的单词数
统计字符串中的单词个数，这里的单词指的是连续的不是空格的字符。

请注意，你可以假定字符串里不包括任何不可打印的字符。

#### 示例:
<pre>
<strong>输入:</strong> "Hello, my name is John"
<strong>输出:</strong> 5
</pre>

## 题解 (Python)

### 1. 计数
```Python3
class Solution:
    def countSegments(self, s: str) -> int:
        cnt = 0

        for i in range(len(s)):
            if not s[i].isspace() and (i == 0 or s[i - 1].isspace()):
                cnt += 1

        return cnt
```
