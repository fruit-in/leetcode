# 2376. Count Special Integers
We call a positive integer **special** if all of its digits are **distinct**.

Given a **positive** integer `n`, return *the number of special integers that belong to the interval* `[1, n]`.

#### Example 1:
<pre>
<strong>Input:</strong> n = 20
<strong>Output:</strong> 19
<strong>Explanation:</strong> All the integers from 1 to 20, except 11, are special. Thus, there are 19 special integers.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 5
<strong>Output:</strong> 5
<strong>Explanation:</strong> All the integers from 1 to 5 are special.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 135
<strong>Output:</strong> 110
<strong>Explanation:</strong> There are 110 integers from 1 to 135 that are special.
Some of the integers that are not special are: 22, 114, and 131.
</pre>

#### Constraints:
* <code>1 <= n <= 2 * 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def countSpecialNumbers(self, n: int) -> int:
        @cache
        def countRemaining(s: str, mask: int) -> int:
            if s == "":
                return 1

            ret = 0

            for i in range(int(s[0])):
                if i == 0 and mask == 0:
                    ret += 9 * sum(362880 // factorial(j)
                                   for j in range(11 - len(s), 10)) + 1
                elif (mask >> i) & 1 == 0:
                    remain = 9 - mask.bit_count()
                    ret += prod(range(remain, remain - len(s) + 1, -1))
            if (mask >> int(s[0])) & 1 == 0:
                ret += countRemaining(s[1:], mask | (1 << int(s[0])))

            return ret

        return countRemaining(str(n), 0) - 1
```
