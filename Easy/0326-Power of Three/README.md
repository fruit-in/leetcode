# 326. Power of Three
Given an integer, write a function to determine if it is a power of three.

#### Example 1:
<pre>
<strong>Input:</strong> 27
<strong>Output:</strong> true
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 0
<strong>Output:</strong> false
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> 9
<strong>Output:</strong> true
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> 45
<strong>Output:</strong> false
</pre>

#### Follow up:
Could you do it without using any loop / recursion?

## Solutions (Python)

### 1. Iteration
```Python3
class Solution:
    def isPowerOfThree(self, n: int) -> bool:
        while n % 3 == 0 and n != 0:
            n //= 3

        return n == 1
```

### 2. Mathematical
```Python3
class Solution:
    def isPowerOfThree(self, n: int) -> bool:
        return n > 0 and 3 ** round(math.log(n, 3)) == n
```

### 3. Integer Limitations
```Python3
class Solution:
    def isPowerOfThree(self, n: int) -> bool:
        return n > 0 and 1162261467 % n == 0
```
