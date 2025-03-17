# 65. Valid Number
Given a string `s`, return whether `s` is a **valid number**.

For example, all the following are valid numbers: `"2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789"`, while the following are not valid numbers: `"abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"`.

Formally, a **valid number** is defined using one of the following definitions:
1. An **integer number** followed by an **optional exponent**.
2. A **decimal number** followed by an **optional exponent**.

An **integer number** is defined with an **optional sign** `'-'` or `'+'` followed by **digits**.

A **decimal number** is defined with an **optional sign** `'-'` or `'+'` followed by one of the following definitions:
1. **Digits** followed by a **dot** `'.'`.
2. **Digits** followed by a **dot** `'.'` followed by **digits**.
3. A **dot** `'.'` followed by **digits**.

An **exponent** is defined with an **exponent notation** `'e'` or `'E'` followed by an **integer number**.

The **digits** are defined as one or more digits.

#### Example 1:
<pre>
<strong>Input:</strong> s = "0"
<strong>Output:</strong> true
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "e"
<strong>Output:</strong> false
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "."
<strong>Output:</strong> false
</pre>

#### Constraints:
* `1 <= s.length <= 20`
* `s` consists of only English letters (both uppercase and lowercase), digits (`0-9`), plus `'+'`, minus `'-'`, or dot `'.'`.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def isNumber(self, s: str) -> bool:
        def isDigits(s: str) -> bool:
            return s != "" and all(c in "0123456789" for c in s)

        def isInteger(s: str) -> bool:
            if s == "":
                return False

            if s[0] in "-+":
                s = s[1:]

            return isDigits(s)

        def isDecimal(s: str) -> bool:
            if s == "":
                return False

            if s[0] in "-+":
                s = s[1:]
            intpart, _, decipart = s.partition('.')

            if isDigits(intpart) and decipart == "":
                return True
            if isDigits(decipart) and intpart == "":
                return True

            return isDigits(intpart) and isDigits(decipart)

        num, e, exp = s.lower().partition('e')

        return (isInteger(num) or isDecimal(num)) and (e == "" or isInteger(exp))
```
