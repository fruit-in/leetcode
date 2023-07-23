# 9. 回文数
判断一个整数是否是回文数。回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。

#### 示例 1:
<pre>
<strong>输入:</strong> 121
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> -121
<strong>输出:</strong> false
<strong>解释:</strong> 从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> 10
<strong>输出:</strong> false
<strong>解释:</strong> 从右向左读, 为 01 。因此它不是一个回文数。
</pre>

#### 进阶:
你能不将整数转为字符串来解决这个问题吗？

## 题解 (Python)

### 1. 双指针
```Python3
class Solution:
    def isPalindrome(self, x: int) -> bool:
        if x < 0:
            return False
        n = 0
        while x // (10 ** n) != 0:
            n += 1
        n -= 1
        while x != 0:
            head = x // (10 ** n)
            tail = x % 10
            if head == tail:
                x %= 10 ** n
                x //= 10
                n -= 2
            else:
                return False
        return True
```

### 2. 反转左半边数字
```Python3
class Solution:
    def isPalindrome(self, x: int) -> bool:
        if x < 0:
            return False
        n = 0
        while x // (10 ** n) != 0:
            n += 1
        if n % 2 == 1:
            left = x // (10 ** (n // 2 + 1))
        else:
            left = x // (10 ** (n // 2))
        right = x % (10 ** (n // 2))
        rev = 0
        while left != 0:
            rev *= 10
            rev += left % 10
            left //= 10
        return rev == right
```

### 3. 反转右半边数字
```Python3
class Solution:
    def isPalindrome(self, x: int) -> bool:
        if x < 0 or ((x % 10) == 0 and x != 0):
            return False
        rev = 0
        while x > rev:
            rev *= 10
            rev += x % 10
            x //= 10
        return x == rev or x == (rev // 10)
```
