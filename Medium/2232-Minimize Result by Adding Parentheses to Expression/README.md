# 2232. Minimize Result by Adding Parentheses to Expression
You are given a **0-indexed** string `expression` of the form `"<num1>+<num2>"` where `<num1>` and `<num2>` represent positive integers.

Add a pair of parentheses to `expression` such that after the addition of parentheses, `expression` is a **valid** mathematical expression and evaluates to the **smallest** possible value. The left parenthesis **must** be added to the left of `'+'` and the right parenthesis **must** be added to the right of `'+'`.

Return `expression` *after adding a pair of parentheses such that* `expression` *evaluates to the **smallest** possible value*. If there are multiple answers that yield the same result, return any of them.

The input has been generated such that the original value of `expression`, and the value of `expression` after adding any pair of parentheses that meets the requirements fits within a signed 32-bit integer.

#### Example 1:
<pre>
<strong>Input:</strong> expression = "247+38"
<strong>Output:</strong> "2(47+38)"
<strong>Explanation:</strong> The expression evaluates to 2 * (47 + 38) = 2 * 85 = 170.
Note that "2(4)7+38" is invalid because the right parenthesis must be to the right of the '+'.
It can be shown that 170 is the smallest possible value.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> expression = "12+34"
<strong>Output:</strong> "1(2+3)4"
<strong>Explanation:</strong> The expression evaluates to 1 * (2 + 3) * 4 = 1 * 5 * 4 = 20.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> expression = "999+999"
<strong>Output:</strong> "(999+999)"
<strong>Explanation:</strong> The expression evaluates to 999 + 999 = 1998.
</pre>

#### Constraints:
* `3 <= expression.length <= 10`
* `expression` consists of digits from `'1'` to `'9'` and `'+'`.
* `expression` starts and ends with digits.
* `expression` contains exactly one `'+'`.
* The original value of `expression`, and the value of `expression` after adding any pair of parentheses that meets the requirements fits within a signed 32-bit integer.

## Solutions (Python)

### 1. Solution
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
