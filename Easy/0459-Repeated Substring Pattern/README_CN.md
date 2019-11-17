# 459. 重复的子字符串
给定一个非空的字符串，判断它是否可以由它的一个子串重复多次构成。给定的字符串只含有小写英文字母，并且长度不超过10000。

#### 示例 1:
<pre>
<strong>输入:</strong> "abab"
<strong>输出:</strong> True
<strong>解释:</strong> 可由子字符串 "ab" 重复两次构成。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "aba"
<strong>输出:</strong> False
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> "abcabcabcabc"
<strong>输出:</strong> True
<strong>解释:</strong> 可由子字符串 "abc" 重复四次构成。 (或者子字符串 "abcabc" 重复两次构成。)
</pre>

## 题解 (Python)

### 1. 题解
```Python3
class Solution:
    def repeatedSubstringPattern(self, s: str) -> bool:
        length = len(s)
        for i in filter(lambda x: length % x == 0, range(1, length // 2 + 1)):
            if s[:i] * (length // i) == s:
                return True
        return False
```

### 2. 题解
```Python3
class Solution:
    def repeatedSubstringPattern(self, s: str) -> bool:
        return s in (s + s)[1:-1]
```
