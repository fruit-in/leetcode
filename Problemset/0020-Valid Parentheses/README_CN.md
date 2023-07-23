# 20. 有效的括号
给定一个只包括 ```'('```，```')'```，```'{'```，```'}'```，```'['```，```']'``` 的字符串，判断字符串是否有效。

有效字符串需满足：
1. 左括号必须用相同类型的右括号闭合
2. 左括号必须以正确的顺序闭合

注意空字符串可被认为是有效字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> "()"
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "()[]{}"
<strong>输出:</strong> true
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> "(]"
<strong>输出:</strong> false
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> "([)]"
<strong>输出:</strong> false
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> "{[]}"
<strong>输出:</strong> true
</pre>

## 题解 (Python)

### 1. 通过栈移除有效括号
```Python3
class Solution:
    def isValid(self, s: str) -> bool:
        if len(s) % 2 != 0:
            return False
        brackets = {')': '(', '}': '{', ']': '['}
        stack = []
        for c in s:
            if c in brackets and stack and stack[-1] == brackets[c]:
                stack.pop()
            else:
                stack.append(c)
        return not stack
```

### 2. 从字符串中移除有效括号
```Python3
class Solution:
    def isValid(self, s: str) -> bool:
        while s.count("()") or s.count("[]") or s.count("{}"):
            s = s.replace("()", "")
            s = s.replace("[]", "")
            s = s.replace("{}", "")
        return not s
```
