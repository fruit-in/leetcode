# 227. 基本计算器 II
给你一个字符串表达式 `s` ，请你实现一个基本计算器来计算并返回它的值。

整数除法仅保留整数部分。

你可以假设给定的表达式总是有效的。所有中间结果将在 <code>[-2<sup>31</sup>, 2<sup>31</sup> - 1]</code> 的范围内。

**注意：**不允许使用任何将字符串作为数学表达式计算的内置函数，比如 `eval()` 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "3+2*2"
<strong>输出:</strong> 7
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = " 3/2 "
<strong>输出:</strong> 1
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = " 3+5 / 2 "
<strong>输出:</strong> 5
</pre>

#### 提示:
* <code>1 <= s.length <= 3 * 10<sup>5</sup></code>
* `s` 由整数和算符 `('+', '-', '*', '/')` 组成，中间由一些空格隔开
* `s` 表示一个 **有效表达式**
* 表达式中的所有整数都是非负整数，且在范围 <code>[0, 2<sup>31</sup> - 1]</code> 内
* 题目数据保证答案是一个 **32-bit 整数**

## 题解 (Python)

### 1. 题解
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
