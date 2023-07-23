# 415. Add Strings
Given two non-negative integers ```num1``` and ```num2``` represented as string, return the sum of ```num1``` and ```num2```.

#### Note:
1. The length of both ```num1``` and ```num2``` is < 5100.
2. Both ```num1``` and ```num2``` contains only digits ```0-9```.
3. Both ```num1``` and ```num2``` does not contain any leading zero.
4. You **must not use any built-in BigInteger library** or **convert the inputs to integer** directly.

## Solutions (Python)

### 1. Straight Forward
```Python3
class Solution:
    def addStrings(self, num1: str, num2: str) -> str:
        sum = ""
        i, j = len(num1) - 1, len(num2) - 1
        carry = 0

        while i >= 0 or j >= 0:
            carry += ord(num1[i]) - 48 if i >= 0 else 0
            carry += ord(num2[j]) - 48 if j >= 0 else 0

            sum += str(carry % 10)
            carry //= 10

            i -= 1
            j -= 1

        return '1' * carry + sum[::-1]
```
