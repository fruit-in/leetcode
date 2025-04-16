# 224. 基本计算器
给你一个字符串表达式 `s` ，请你实现一个基本计算器来计算并返回它的值。

注意:不允许使用任何将字符串作为数学表达式计算的内置函数，比如 `eval()` 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "1 + 1"
<strong>输出:</strong> 2
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = " 2-1 + 2 "
<strong>输出:</strong> 3
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "(1+(4+5+2)-3)+(6+8)"
<strong>输出:</strong> 23
</pre>

#### 提示:
* <code>1 <= s.length <= 3 * 10<sup>5</sup></code>
* `s` 由数字、`'+'`、`'-'`、`'('`、`')'`、和 `' '` 组成
* `s` 表示一个有效的表达式
* `'+'` 不能用作一元运算(例如， `"+1"` 和 `"+(2 + 3)"` 无效)
* `'-'` 可以用作一元运算(即 `"-1"` 和 `"-(2 + 3)"` 是有效的)
* 输入中不存在两个连续的操作符
* 每个数字和运行的计算将适合于一个有符号的 32位 整数

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def calculate(self, s: str) -> int:
        s = s.replace(' ', '')
        negstack = [False]
        stack = []
        ret = 0

        for i in range(len(s)):
            if s[i] == '(':
                if i == 0 or s[i - 1] != '-':
                    negstack.append(negstack[-1])
                else:
                    negstack.append(not negstack[-1])
            elif s[i] == ')':
                negstack.pop()
            elif s[i] == '+':
                stack.append('-' if negstack[-1] else '+')
            elif s[i] == '-':
                if stack == [] or isinstance(stack[-1], str):
                    stack.append(0)
                stack.append('+' if negstack[-1] else '-')
            else:
                if stack == [] or isinstance(stack[-1], str):
                    stack.append(0)
                stack[-1] = stack[-1] * 10 + int(s[i])

        ret = stack[0]

        for i in range(2, len(stack), 2):
            ret += stack[i] if stack[i - 1] == '+' else -stack[i]

        return ret
```
