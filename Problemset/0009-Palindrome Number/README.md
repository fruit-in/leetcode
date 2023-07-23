# 9. Palindrome Number
Determine whether an integer is a palindrome. An integer is a palindrome when it reads the same backward as forward.

#### Example 1:
<pre>
<strong>Input:</strong> 121
<strong>Output:</strong> true
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> -121
<strong>Output:</strong> false
<strong>Explanation:</strong> From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a
palindrome.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> 10
<strong>Output:</strong> false
<strong>Explanation:</strong> Reads 01 from right to left. Therefore it is not a palindrome.
</pre>

#### Follow up:
Coud you solve it without converting the integer to a string?

## Solutions (Python)

### 1. Two Points
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

### 2. Revert Left Half
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

### 3. Revert Right Half
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

## Solutions (C)
### 1. Elementary Math
```C
bool isPalindrome(int x){
    int origin = x;
    long sum = 0;
    while(x > 0)
    {
        sum = sum *10 + x % 10;
        x /= 10;
    }    
    if(sum == origin)
    {
        return true;
    }
    return false;
}
```
