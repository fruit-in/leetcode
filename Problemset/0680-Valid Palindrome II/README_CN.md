# 680. 验证回文字符串 Ⅱ
给定一个非空字符串 ```s```，**最多**删除一个字符。判断是否能成为回文字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> "aba"
<strong>输出:</strong> True
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "abca"
<strong>输出:</strong> True
<strong>解释:</strong> 你可以删除c字符。
</pre>

#### 注意:
1. 字符串只包含从 a-z 的小写字母。字符串的最大长度是50000。

## 题解 (Python)

### 1. 双指针
```Python3
class Solution:
    def validPalindrome(self, s: str) -> bool:
        def isPalindrome(l: int, r: int) -> bool:
            return all(s[l + i] == s[r - i] for i in range((r - l + 1) // 2))

        l, r = 0, len(s) - 1
        while l < r:
            if s[l] != s[r]:
                return isPalindrome(l + 1, r) or isPalindrome(l, r - 1)

            l += 1
            r -= 1

        return True
```
