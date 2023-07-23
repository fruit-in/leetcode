# 20. Valid Parentheses
Given a string containing just the characters <code>'('</code>, <code>')'</code>, <code>'{'</code>, <code>'}'</code>, <code>'['</code> and <code>']'</code>, determine if the input string is valid.

An input string is valid if:
1. Open brackets must be closed by the same type of brackets.
2. Open brackets must be closed in the correct order.

Note that an empty string is also considered valid.

#### Example 1:
<pre>
<strong>Input:</strong> "()"
<strong>Output:</strong> true
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "()[]{}"
<strong>Output:</strong> true
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> "(]"
<strong>Output:</strong> false
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> "([)]"
<strong>Output:</strong> false
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> "{[]}"
<strong>Output:</strong> true
</pre>

## Solutions (Python)

### 1. Remove Valid Parentheses by Stack
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

### 2. Remove Valid Parentheses from String
```Python3
class Solution:
    def isValid(self, s: str) -> bool:
        while s.count("()") or s.count("[]") or s.count("{}"):
            s = s.replace("()", "")
            s = s.replace("[]", "")
            s = s.replace("{}", "")
        return not s
```
