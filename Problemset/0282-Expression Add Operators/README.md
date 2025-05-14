# 282. Expression Add Operators
Given a string `num` that contains only digits and an integer `target`, return ***all possibilities** to insert the binary operators* `'+'`, `'-'`*, and/or* `'*'` *between the digits of* `num` *so that the resultant expression evaluates to the* `target` *value*.

Note that operands in the returned expressions **should not** contain leading zeros.

#### Example 1:
<pre>
<strong>Input:</strong> num = "123", target = 6
<strong>Output:</strong> ["1*2*3","1+2+3"]
<strong>Explanation:</strong> Both "1*2*3" and "1+2+3" evaluate to 6.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num = "232", target = 8
<strong>Output:</strong> ["2*3+2","2+3*2"]
<strong>Explanation:</strong> Both "2*3+2" and "2+3*2" evaluate to 8.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> num = "3456237490", target = 9191
<strong>Output:</strong> []
<strong>Explanation:</strong> There are no expressions that can be created from "3456237490" to evaluate to 9191.
</pre>

#### Constraints:
* `1 <= num.length <= 10`
* `num` consists of only digits.
* <code>-2<sup>31</sup> <= target <= 2<sup>31</sup> - 1</code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def addOperators(self, num: str, target: int) -> List[str]:
        def dfs(i: int) -> None:
            if i == len(chars):
                expression = ''.join(chars)
                if pattern.search(expression) is None and eval(expression) == target:
                    ret.append(expression)
                return

            chars[i] = '+'
            dfs(i + 2)
            chars[i] = '-'
            dfs(i + 2)
            chars[i] = '*'
            dfs(i + 2)
            chars[i] = ''
            dfs(i + 2)

        chars = [num[0]]
        pattern = re.compile(r'(?<!\d)0\d+')
        ret = []

        for c in num[1:]:
            chars.append('')
            chars.append(c)

        dfs(1)

        return ret
```
