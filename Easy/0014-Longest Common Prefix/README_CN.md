# 14. 最长公共前缀
编写一个函数来查找字符串数组中的最长公共前缀。

如果不存在公共前缀，返回空字符串 ```""```。

#### 示例 1:
<pre>
<strong>输入:</strong> ["flower","flow","flight"]
<strong>输出:</strong> "fl"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> ["dog","racecar","car"]
<strong>输出:</strong> ""
<strong>解释:</strong> 输入不存在公共前缀。
</pre>

#### 说明:
所有输入只包含小写字母 ```a-z```。

## 题解 (Python)

### 1. 二分查找
```Python3
class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        prefix = ''
        if strs:
            maxlength = len(strs[0])
            index = maxlength
            while len(prefix) != index:
                for s in strs[1:]:
                    if not s.startswith(strs[0][:index]):
                        maxlength = index
                        index = maxlength // 2
                        break
                else:
                    prefix = strs[0][:index]
                    index = (index + maxlength) // 2
        return prefix
```

### 2. 分治
```Python3
class Solution:
    def longestCommonPrefix(self, strs):
        if len(strs) == 0:
            return ''
        elif len(strs) == 1:
            return strs[0]
        else:
            prefix1 = self.longestCommonPrefix(strs[:len(strs) // 2])
            prefix2 = self.longestCommonPrefix(strs[len(strs) // 2:])
            for i in range(len(prefix1) if len(prefix1) < len(prefix2) else len(prefix2)):
                if prefix1[i] != prefix2[i]:
                    return prefix1[:i]
            return prefix1[:len(prefix2)]
```

### 3. 水平扫描
```Python3
class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        if not strs:
            return ''
        return self.findPrefix(strs[1:], strs[0])

    def findPrefix(self, strs: List[str], prefix: str) -> str:
        for k, v in enumerate(strs):
            if not v.startswith(prefix):
                return self.findPrefix(strs[k:], prefix[:-1])
        return prefix
```

### 4. 垂直扫描
```Python3
class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        prefix = ''
        if strs:
            for i, c in enumerate(strs[0]):
                for s in strs[1:]:
                    if i == len(s) or s[i] != c:
                        return prefix
                prefix = strs[0][:i + 1]
        return prefix
```
