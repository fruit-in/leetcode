# 1416. Restore The Array
A program was supposed to print an array of integers. The program forgot to print whitespaces and the array is printed as a string of digits `s` and all we know is that all integers in the array were in the range `[1, k]` and there are no leading zeros in the array.

Given the string `s` and the integer `k`, return *the number of the possible arrays that can be printed as* `s` *using the mentioned program*. Since the answer may be very large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> s = "1000", k = 10000
<strong>Output:</strong> 1
<strong>Explanation:</strong> The only possible array is [1000]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "1000", k = 10
<strong>Output:</strong> 0
<strong>Explanation:</strong> There cannot be an array that was printed this way and has all integer >= 1 and <= 10.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "1317", k = 2000
<strong>Output:</strong> 8
<strong>Explanation:</strong> Possible arrays are [1317],[131,7],[13,17],[1,317],[13,1,7],[1,31,7],[1,3,17],[1,3,1,7]
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` consists of only digits and does not contain leading zeros.
* <code>1 <= k <= 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def numberOfArrays(self, s: str, k: int) -> int:
        dp = [0] * (len(s) + 1)
        dp[0] = 1

        for i in range(len(s)):
            if s[i] == '0':
                continue

            x = int(s[i])
            j = 1

            while x <= k and i + j <= len(s):
                dp[i + j] = (dp[i + j] + dp[i]) % 1000000007
                if i + j < len(s):
                    x = x * 10 + int(s[i + j])
                j += 1

        return dp[-1]
```
