# 2507. Smallest Value After Replacing With Sum of Prime Factors
You are given a positive integer `n`.

Continuously replace `n` with the sum of its **prime factors**.

* Note that if a prime factor divides `n` multiple times, it should be included in the sum as many times as it divides `n`.

Return *the smallest value* `n` *will take on*.

#### Example 1:
<pre>
<strong>Input:</strong> n = 15
<strong>Output:</strong> 5
<strong>Explanation:</strong> Initially, n = 15.
15 = 3 * 5, so replace n with 3 + 5 = 8.
8 = 2 * 2 * 2, so replace n with 2 + 2 + 2 = 6.
6 = 2 * 3, so replace n with 2 + 3 = 5.
5 is the smallest value n will take on.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> Initially, n = 3.
3 is the smallest value n will take on.
</pre>

#### Constraints:
* <code>2 <= n <= 10<sup>5</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    lpf = [0] * 100001
    for i in range(2, 100001):
        if lpf[i] == 0:
            for j in range(i, 100001, i):
                if lpf[j] == 0:
                    lpf[j] = i

    def smallestValue(self, n: int) -> int:
        while self.lpf[n] != n:
            x = n
            pfsum = 0

            while x > 1:
                pfsum += self.lpf[x]
                x //= self.lpf[x]

            if n == pfsum:
                break

            n = pfsum

        return n
```
