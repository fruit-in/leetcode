# 290. 单词规律
给定一种规律 ```pattern``` 和一个字符串 ```str``` ，判断 ```str``` 是否遵循相同的规律。

这里的 **遵循** 指完全匹配，例如， ```pattern``` 里的每个字母和字符串 ```str``` 中的每个非空单词之间存在着双向连接的对应规律。

#### 示例 1:
<pre>
<strong>输入:</strong> pattern = "abba", str = "dog cat cat dog"
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> pattern = "abba", str = "dog cat cat fish"
<strong>输出:</strong> false
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> pattern = "aaaa", str = "dog cat cat dog"
<strong>输出:</strong> false
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> pattern = "abba", str = "dog dog dog dog"
<strong>输出:</strong> false
</pre>

#### 说明:
你可以假设 ```pattern``` 只包含小写字母， ```str``` 包含了由单个空格分隔的小写字母。

## 题解 (Python)

### 1. 暴力法
```Python3
class Solution:
    def wordPattern(self, pattern: str, str: str) -> bool:
        words = str.split()

        if len(pattern) != len(words):
            return False

        for i in range(len(pattern)):
            for j in range(i + 1, len(pattern)):
                if (pattern[j] == pattern[i]) != (words[j] == words[i]):
                    return False

        return True
```

### 2. 哈希表
```Python3
class Solution:
    def wordPattern(self, pattern: str, str: str) -> bool:
        words = str.split()

        if len(pattern) != len(words):
            return False

        match = {}
        used = set()

        for ch, wo in zip(pattern, words):
            if (ch in match) != (wo in used):
                return False
            elif ch not in match:
                match[ch] = wo
                used.add(wo)
            elif match[ch] != wo:
                return False

        return True
```
