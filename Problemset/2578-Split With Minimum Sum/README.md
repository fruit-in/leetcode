# 2578. Split With Minimum Sum
Given a positive integer `num`, split it into two non-negative integers `num1` and `num2` such that:

* The concatenation of `num1` and `num2` is a permutation of `num`.
    * In other words, the sum of the number of occurrences of each digit in `num1` and `num2` is equal to the number of occurrences of that digit in `num`.
* `num1` and `num2` can contain leading zeros.

Return *the **minimum** possible sum of* `num1` *and* `num2`.

**Notes:**

* It is guaranteed that `num` does not contain any leading zeros.
* The order of occurrence of the digits in `num1` and `num2` may differ from the order of occurrence of `num`.

#### Example 1:
<pre>
<strong>Input:</strong> num = 4325
<strong>Output:</strong> 59
<strong>Explanation:</strong> We can split 4325 so that num1 is 24 and num2 is 35, giving a sum of 59. We can prove that 59 is indeed the minimal possible sum.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num = 687
<strong>Output:</strong> 75
<strong>Explanation:</strong> We can split 687 so that num1 is 68 and num2 is 7, which would give an optimal sum of 75.
</pre>

#### Constraints:
* <code>10 <= num <= 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def splitNum(self, num: int) -> int:
        num = sorted(str(num))
        num1 = int(''.join(num[::2]))
        num2 = int(''.join(num[1::2]))

        return num1 + num2
```
