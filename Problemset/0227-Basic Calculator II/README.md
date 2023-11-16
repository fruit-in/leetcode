# 227. Basic Calculator II
Given a string `s` which represents an expression, *evaluate this expression and return its value*.

The integer division should truncate toward zero.

You may assume that the given expression is always valid. All intermediate results will be in the range of <code>[-2<sup>31</sup>, 2<sup>31</sup> - 1]</code>.

**Note:** You are not allowed to use any built-in function which evaluates strings as mathematical expressions, such as `eval()`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "3+2*2"
<strong>Output:</strong> 7
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = " 3/2 "
<strong>Output:</strong> 1
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = " 3+5 / 2 "
<strong>Output:</strong> 5
</pre>

#### Constraints:
* <code>1 <= s.length <= 3 * 10<sup>5</sup></code>
* `s` consists of integers and operators `('+', '-', '*', '/')` separated by some number of spaces.
* `s` represents **a valid expression**.
* All the integers in the expression are non-negative integers in the range <code>[0, 2<sup>31</sup> - 1]</code>.
* The answer is **guaranteed** to fit in a **32-bit integer**.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def calculate(self, s: str) -> int:
        isint = False
        lastsign = None
        stack = []
        ret = 0

        for ch in s:
            if ch in "+-*/":
                if lastsign == '*':
                    x, _ = stack.pop(), stack.pop()
                    stack[-1] *= x
                elif lastsign == '/':
                    x, _ = stack.pop(), stack.pop()
                    stack[-1] //= x

                isint = False
                lastsign = ch
                stack.append(ch)
            elif ch.isdigit():
                if isint:
                    stack[-1] = stack[-1] * 10 + int(ch)
                else:
                    isint = True
                    stack.append(int(ch))

        if lastsign == '*':
            x, _ = stack.pop(), stack.pop()
            stack[-1] *= x
        elif lastsign == '/':
            x, _ = stack.pop(), stack.pop()
            stack[-1] //= x

        lastsign = '+'

        for elem in stack:
            if isinstance(elem, int):
                if lastsign == '+':
                    ret += elem
                elif lastsign == '-':
                    ret -= elem
            else:
                lastsign = elem

        return ret
```
