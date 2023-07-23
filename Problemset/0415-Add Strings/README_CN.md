# 415. 字符串相加
给定两个字符串形式的非负整数 ```num1``` 和```num2``` ，计算它们的和。

#### 注意:
1. ```num1``` 和```num2``` 的长度都小于 5100.
2. ```num1``` 和```num2``` 都只包含数字 ```0-9```.
3. ```num1``` 和```num2``` 都不包含任何前导零。
4. **你不能使用任何內建 BigInteger 库， 也不能直接将输入的字符串转换为整数形式。**

## 题解 (Python)

### 1. 题解
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
