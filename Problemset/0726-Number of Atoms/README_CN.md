# 726. 原子的数量
给你一个字符串化学式 `formula` ，返回 **每种原子的数量** 。

原子总是以一个大写字母开始，接着跟随 0 个或任意个小写字母，表示原子的名字。

如果数量大于 1，原子后会跟着数字表示原子的数量。如果数量等于 1 则不会跟数字。

* 例如，`"H2O"` 和 `"H2O2"` 是可行的，但 `"H1O2"` 这个表达是不可行的。

两个化学式连在一起可以构成新的化学式。

* 例如 `"H2O2He3Mg4"` 也是化学式。

由括号括起的化学式并佐以数字（可选择性添加）也是化学式。

* 例如 `"(H2O2)"` 和 `"(H2O2)3"` 是化学式。

返回所有原子的数量，格式为：第一个（按字典序）原子的名字，跟着它的数量（如果数量大于 1），然后是第二个原子的名字（按字典序），跟着它的数量（如果数量大于 1），以此类推。

#### 示例 1:
<pre>
<strong>输入:</strong> formula = "H2O"
<strong>输出:</strong> "H2O"
<strong>解释:</strong> 原子的数量是 {'H': 2, 'O': 1}。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> formula = "Mg(OH)2"
<strong>输出:</strong> "H2MgO2"
<strong>解释:</strong> 原子的数量是 {'H': 2, 'Mg': 1, 'O': 2}。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> formula = "K4(ON(SO3)2)2"
<strong>输出:</strong> "K4N2O14S4"
<strong>解释:</strong> 原子的数量是 {'K': 4, 'N': 2, 'O': 14, 'S': 4}。
</pre>

#### 提示:
* `1 <= formula.length <= 1000`
* `formula` 由英文字母、数字、`'('` 和 `')'` 组成
* `formula` 总是有效的化学式

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def countOfAtoms(self, formula: str) -> str:
        stack = []
        factors = []
        x = 1
        counter = {}
        ret = []

        for i, c in enumerate(formula):
            if c.islower():
                stack[-1][0] += c
            elif c.isdigit():
                if not formula[i - 1].isdigit():
                    stack[-1][1] = 0
                stack[-1][1] = stack[-1][1] * 10 + int(c)
            else:
                stack.append([c, 1])

        while stack:
            atom, count = stack.pop()

            if atom == ')':
                factors.append(count)
                x *= count
            elif atom == '(':
                x //= factors.pop()
            else:
                if atom not in counter:
                    counter[atom] = 0
                counter[atom] += count * x

        for atom, count in sorted(counter.items()):
            if count > 1:
                ret.append(atom + str(count))
            else:
                ret.append(atom)

        return ''.join(ret)
```
