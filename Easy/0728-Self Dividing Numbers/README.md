# 728. Self Dividing Numbers
A *self-dividing number* is a number that is divisible by every digit it contains.

For example, 128 is a self-dividing number because <code>128 % 1 == 0</code>, <code>128 % 2 == 0</code>, and <code>128 % 8 == 0</code>.

Also, a self-dividing number is not allowed to contain the digit zero.

Given a lower and upper number bound, output a list of every possible self dividing number, including the bounds if possible.

#### Example 1:
<pre>
<strong>Input:</strong> left = 1, right = 22
<strong>Output:</strong> [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
</pre>

#### Note:
* The boundaries of each input argument are <code>1 <= left <= right <= 10000</code>.

## Solutions (Python)

### 1. Solution
```Python3
class Solution:
    def selfDividingNumbers(self, left: int, right: int) -> List[int]:
        return list(filter(self.isSelfDividingNumber, range(left, right + 1)))
    
    def isSelfDividingNumber(self, num: int) -> bool:
        if num == 0:
            return False
        n = num
        while num != 0:
            if num % 10 == 0 or n % (num % 10) != 0:
                return False
            num //= 10
        return True
```
