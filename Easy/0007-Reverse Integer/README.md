# 7. Reverse Integer
Given a 32-bit signed integer, reverse digits of an integer.

#### Example 1:
<pre>
<strong>Input:</strong> 123
<strong>Output:</strong> 321
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> -123
<strong>Output:</strong> -321
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> 120
<strong>Output:</strong> 21
</pre>

#### Note:
Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−2<sup>31</sup>, 2<sup>31</sup> − 1]. For the purpose of this problem, assume that your function returns 0 when the reversed integer overflows.

## Solutions

### 1. Pop & Push Digits (Python3)
```Python3
class Solution:
    def reverse(self, x: int) -> int:
        result = 0
        xc = abs(x)
        while xc != 0:
            result *= 10
            result += xc % 10
            xc //= 10
        if result < -(2 ** 31) or result > (2 ** 31) - 1:
            return 0
        elif x >= 0:
            return result
        else:
            return -result
```
