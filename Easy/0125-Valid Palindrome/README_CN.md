# 125. 验证回文串
给定一个字符串，验证它是否是回文串，只考虑字母和数字字符，可以忽略字母的大小写。

**说明:** 本题中，我们将空字符串定义为有效的回文串。

#### 示例 1:
<pre>
<strong>输入:</strong> "A man, a plan, a canal: Panama"
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "race a car"
<strong>输出:</strong> false
</pre>

## 题解 (Python)

### 1. 双指针
```Python3
class Solution:
    def isPalindrome(self, s: str) -> bool:
        s = ''.join([c for c in s if c.isalnum()]).lower()
        i, j = 0, len(s) - 1
        while i < j:
            if s[i] != s[j]:
                return False
            i += 1
            j -= 1
        return True
```

### 2. 反转字符串
```Python3
class Solution:
    def isPalindrome(self, s: str) -> bool:
        s = ''.join([c for c in s if c.isalnum()]).lower()
        return s[::-1] == s
```
