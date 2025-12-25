# 726. Number of Atoms
Given a string `formula` representing a chemical formula, return *the count of each atom*.

The atomic element always starts with an uppercase character, then zero or more lowercase letters, representing the name.

One or more digits representing that element's count may follow if the count is greater than `1`. If the count is `1`, no digits will follow.

* For example, `"H2O"` and `"H2O2"` are possible, but `"H1O2"` is impossible.

Two formulas are concatenated together to produce another formula.

* For example, `"H2O2He3Mg4"` is also a formula.

A formula placed in parentheses, and a count (optionally added) is also a formula.

* For example, `"(H2O2)"` and `"(H2O2)3"` are formulas.

Return the count of all elements as a string in the following form: the first name (in sorted order), followed by its count (if that count is more than `1`), followed by the second name (in sorted order), followed by its count (if that count is more than `1`), and so on.

The test cases are generated so that all the values in the output fit in a **32-bit** integer.

#### Example 1:
<pre>
<strong>Input:</strong> formula = "H2O"
<strong>Output:</strong> "H2O"
<strong>Explanation:</strong> The count of elements are {'H': 2, 'O': 1}.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> formula = "Mg(OH)2"
<strong>Output:</strong> "H2MgO2"
<strong>Explanation:</strong> The count of elements are {'H': 2, 'Mg': 1, 'O': 2}.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> formula = "K4(ON(SO3)2)2"
<strong>Output:</strong> "K4N2O14S4"
<strong>Explanation:</strong> The count of elements are {'K': 4, 'N': 2, 'O': 14, 'S': 4}.
</pre>

#### Constraints:
* `1 <= formula.length <= 1000`
* `formula` consists of English letters, digits, `'('`, and `')'`.
* `formula` is always valid.

## Solutions (Python)

### 1. Solution
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
