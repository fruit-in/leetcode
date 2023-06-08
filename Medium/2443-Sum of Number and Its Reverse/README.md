# 2443. Sum of Number and Its Reverse
Given a **non-negative** integer `num`, return `true` *if* `num` *can be expressed as the sum of any **non-negative** integer and its reverse, or* `false` *otherwise*.

#### Example 1:
<pre>
<strong>Input:</strong> num = 443
<strong>Output:</strong> true
<strong>Explanation:</strong> 172 + 271 = 443 so we return true.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num = 63
<strong>Output:</strong> false
<strong>Explanation:</strong> 63 cannot be expressed as the sum of a non-negative integer and its reverse so we return false.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> num = 181
<strong>Output:</strong> true
<strong>Explanation:</strong> 140 + 041 = 181 so we return true. Note that when a number is reversed, there may be leading zeros.
</pre>

#### Constraints:
* <code>0 <= num <= 10<sup>5</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def sumOfNumberAndReverse(self, num: int) -> bool:
        return any(k + int(str(k)[::-1]) == num for k in range(num + 1))
```
