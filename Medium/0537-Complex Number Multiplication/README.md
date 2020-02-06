# 537. Complex Number Multiplication
Given two strings representing two [complex numbers](https://en.wikipedia.org/wiki/Complex_number).

You need to return a string representing their multiplication. Note i<sup>2</sup> = -1 according to the definition.

#### Example 1:
<pre>
<strong>Input:</strong> "1+1i", "1+1i"
<strong>Output:</strong> "0+2i"
<strong>Explanation:</strong> (1 + i) * (1 + i) = 1 + i<sup>2</sup> + 2 * i = 2i, and you need convert it to the form of 0+2i.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "1+-1i", "1+-1i"
<strong>Output:</strong> "0+-2i"
<strong>Explanation:</strong> (1 - i) * (1 - i) = 1 + i<sup>2</sup> - 2 * i = -2i, and you need convert it to the form of 0+-2i.
</pre>

#### Note:
1. The input strings will not have extra blank.
2. The input strings will be given in the form of **a+bi**, where the integer **a** and **b** will both belong to the range of [-100, 100]. And **the output should be also in this form**.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def complexNumberMultiply(self, a: str, b: str) -> str:
        a = [int(x) for x in a[:-1].split('+')]
        b = [int(x) for x in b[:-1].split('+')]

        c = a[0] * b[0] - a[1] * b[1]
        d = a[0] * b[1] + a[1] * b[0]

        return "%d+%di" % (c, d)
```
