# 479. 最大回文数乘积
给定一个整数 n ，返回 *可表示为两个 `n` 位整数乘积的 **最大回文整数*** 。因为答案可能非常大，所以返回它对 `1337` **取余** 。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 2
<strong>输出:</strong> 987
<strong>解释:</strong> 99 x 91 = 9009, 9009 % 1337 = 987
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 1
<strong>输出:</strong> 9
</pre>

#### 提示:
* `1 <= n <= 8`

## 题解 (Python)

### 1. 题解
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
