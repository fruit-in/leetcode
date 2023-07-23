# 2180. Count Integers With Even Digit Sum
Given a positive integer `num`, return *the number of positive integers **less than or equal to*** `num` *whose digit sums are **even***.

The **digit sum** of a positive integer is the sum of all its digits.

#### Example 1:
<pre>
<strong>Input:</strong> num = 4
<strong>Output:</strong> 2
<strong>Explanation:</strong>
The only integers less than or equal to 4 whose digit sums are even are 2 and 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num = 30
<strong>Output:</strong> 14
<strong>Explanation:</strong> The 14 integers less than or equal to 30 whose digit sums are even are
2, 4, 6, 8, 11, 13, 15, 17, 19, 20, 22, 24, 26, and 28.
</pre>

#### Constraints:
* `1 <= num <= 1000`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def countEven(self, num: int) -> int:
        return sum(1 for x in range(2, num + 1)
                   if sum([x // 1000, x % 1000 // 100, x % 100 // 10, x % 10]) % 2 == 0)
```
