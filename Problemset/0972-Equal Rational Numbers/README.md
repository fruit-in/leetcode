# 972. Equal Rational Numbers
Given two strings `s` and `t`, each of which represents a non-negative rational number, return `true` if and only if they represent the same number. The strings may use parentheses to denote the repeating part of the rational number.

A **rational number** can be represented using up to three parts: `<IntegerPart>`, `<NonRepeatingPart>`, and a `<RepeatingPart>`. The number will be represented in one of the following three ways:
* `<IntegerPart>`
    * For example, `12`, `0`, and `123`.
* `<IntegerPart><.><NonRepeatingPart>`
    * For example, `0.5`, `1.`, `2.12`, and `123.0001`.
* `<IntegerPart><.><NonRepeatingPart><(><RepeatingPart><)>`
    * For example, `0.1(6)`, `1.(9)`, `123.00(1212)`.

The repeating portion of a decimal expansion is conventionally denoted within a pair of round brackets. For example:
* `1/6 = 0.16666666... = 0.1(6) = 0.1666(6) = 0.166(66)`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "0.(52)", t = "0.5(25)"
<strong>Output:</strong> true
<strong>Explanation:</strong> Because "0.(52)" represents 0.52525252..., and "0.5(25)" represents 0.52525252525..... , the strings represent the same number.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "0.1666(6)", t = "0.166(66)"
<strong>Output:</strong> true
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "0.9(9)", t = "1."
<strong>Output:</strong> true
<strong>Explanation:</strong> "0.9(9)" represents 0.999999999... repeated forever, which equals 1.  [<a href="https://en.wikipedia.org/wiki/0.999...">See this link for an explanation.</a>]
"1." represents the number 1, which is formed correctly: (IntegerPart) = "1" and (NonRepeatingPart) = "".
</pre>

#### Constraints:
* Each part consists only of digits.
* The `<IntegerPart>` does not have leading zeros (except for the zero itself).
* `1 <= <IntegerPart>.length <= 4`
* `0 <= <NonRepeatingPart>.length <= 4`
* `1 <= <RepeatingPart>.length <= 4`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def isRationalEqual(self, s: str, t: str) -> bool:
        def toFraction(s: str) -> (int, int):
            s = re.split(r"[.()]", s)
            integer = int(s[0])
            nonrepeating = int(s[1]) if len(s) > 1 and s[1] != '' else None
            repeating = int(s[2]) if len(s) > 2 else None

            if nonrepeating is None and repeating is None:
                denominator = 1
                numerator = 0
            elif nonrepeating is None:
                denominator = 10 ** len(s[2]) - 1
                numerator = repeating
            elif repeating is None:
                denominator = 10 ** len(s[1])
                numerator = nonrepeating
            else:
                denominator = 10 ** (len(s[1]) + len(s[2])) - 10 ** (len(s[1]))
                numerator = nonrepeating * (10 ** len(s[2]) - 1) + repeating

            numerator += denominator * integer
            gcdnum = gcd(numerator, denominator)

            return (numerator // gcdnum, denominator // gcdnum)

        return toFraction(s) == toFraction(t)
```
