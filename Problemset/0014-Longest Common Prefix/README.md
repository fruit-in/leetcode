# 14. Longest Common Prefix
Write a function to find the longest common prefix string amongst an array of strings.

If there is no common prefix, return an empty string <code>""</code>.

#### Example 1:
<pre>
<strong>Input:</strong> ["flower","flow","flight"]
<strong>Output:</strong> "fl"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> ["dog","racecar","car"]
<strong>Output:</strong> ""
<strong>Explanation:</strong> There is no common prefix among the input strings.
</pre>

#### Note:
All given inputs are in lowercase letters <code>a-z</code>.

## Solutions (Python)

### 1. Binary Search
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

### 2. Divide & Conquer
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

### 3. Horizontal Scanning
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

### 4. Vertical Scanning
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
