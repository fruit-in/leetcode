# 224. Basic Calculator
Given a string `s` representing a valid expression, implement a basic calculator to evaluate it, and return *the result of the evaluation*.

**Note:** You are **not** allowed to use any built-in function which evaluates strings as mathematical expressions, such as `eval()`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "1 + 1"
<strong>Output:</strong> 2
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = " 2-1 + 2 "
<strong>Output:</strong> 3
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "(1+(4+5+2)-3)+(6+8)"
<strong>Output:</strong> 23
</pre>

#### Constraints:
* <code>1 <= s.length <= 3 * 10<sup>5</sup></code>
* `s` consists of digits, `'+'`, `'-'`, `'('`, `')'`, and `' '`.
* `s` represents a valid expression.
* `'+'` is **not** used as a unary operation (i.e., `"+1"` and `"+(2 + 3)"` is invalid).
* `'-'` could be used as a unary operation (i.e., `"-1"` and `"-(2 + 3)"` is valid).
* There will be no two consecutive operators in the input.
* Every number and running calculation will fit in a signed 32-bit integer.

## Solutions (Python)

### 1. Solution
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
