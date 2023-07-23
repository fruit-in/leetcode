# 2232. 向表达式添加括号后的最小结果
给你一个下标从 **0** 开始的字符串 `expression` ，格式为 `"<num1>+<num2>"` ，其中 `<num1>` 和 `<num2>` 表示正整数。

请你向 `expression` 中添加一对括号，使得在添加之后， `expression` 仍然是一个有效的数学表达式，并且计算后可以得到 **最小** 可能值。左括号 **必须** 添加在 `'+'` 的左侧，而右括号必须添加在 `'+'` 的右侧。

返回添加一对括号后形成的表达式 `expression` ，且满足 `expression` 计算得到 **最小** 可能值。如果存在多个答案都能产生相同结果，返回任意一个答案。

生成的输入满足：`expression` 的原始值和添加满足要求的任一对括号之后 `expression` 的值，都符合 32-bit 带符号整数范围。

#### 示例 1:
<pre>
<strong>输入:</strong> expression = "247+38"
<strong>输出:</strong> "2(47+38)"
<strong>解释:</strong> 表达式计算得到 2 * (47 + 38) = 2 * 85 = 170 。
注意 "2(4)7+38" 不是有效的结果，因为右括号必须添加在 '+' 的右侧。
可以证明 170 是最小可能值。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> expression = "12+34"
<strong>输出:</strong> "1(2+3)4"
<strong>解释:</strong> 表达式计算得到 1 * (2 + 3) * 4 = 1 * 5 * 4 = 20 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> expression = "999+999"
<strong>输出:</strong> "(999+999)"
<strong>解释:</strong> 表达式计算得到 999 + 999 = 1998 。
</pre>

#### 提示:
* `3 <= expression.length <= 10`
* `expression` 仅由数字 `'1'` 到 `'9'` 和 `'+'` 组成
* `expression` 由数字开始和结束
* `expression` 恰好仅含有一个 `'+'`.
* `expression` 的原始值和添加满足要求的任一对括号之后 `expression` 的值，都符合 32-bit 带符号整数范围

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def minimizeResult(self, expression: str) -> str:
        m = expression.find("+")
        smallest = float("inf")
        ret = ""

        for l in range(m):
            for r in range(m + 2, len(expression) + 1):
                x = int(expression[l:m]) + int(expression[m + 1:r])
                if l > 0:
                    x *= int(expression[:l])
                if r < len(expression):
                    x *= int(expression[r:])

                if x < smallest:
                    smallest = x
                    ret = expression[:l] + \
                        "(" + expression[l:r] + ")" + expression[r:]

        return ret
```
