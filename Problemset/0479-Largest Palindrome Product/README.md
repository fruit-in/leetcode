# 479. Largest Palindrome Product
Given an integer n, return *the **largest palindromic integer** that can be represented as the product of two `n`-digits integers*. Since the answer can be very large, return it **modulo** `1337`.

#### Example 1:
<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> 987
<strong>Explanation:</strong> 99 x 91 = 9009, 9009 % 1337 = 987
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 9
</pre>

#### Constraints:
* `1 <= n <= 8`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def largestPalindrome(self, n: int) -> int:
        if n == 1:
            return 9

        maxx, minx = 10 ** n - 1, 10 ** (n - 1)
        for x in range(int(str(maxx ** 2)[:n]), minx - 1, -1):
            palindrome = int(str(x) + str(x)[::-1])
            for y in range(maxx, int(sqrt(palindrome)) - 1, -1):
                if palindrome % y == 0:
                    return palindrome % 1337
```
