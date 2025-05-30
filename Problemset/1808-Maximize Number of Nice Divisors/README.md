# 1808. Maximize Number of Nice Divisors
You are given a positive integer `primeFactors`. You are asked to construct a positive integer `n` that satisfies the following conditions:
* The number of prime factors of `n` (not necessarily distinct) is **at most** `primeFactors`.
* The number of nice divisors of `n` is maximized. Note that a divisor of `n` is **nice** if it is divisible by every prime factor of `n`. For example, if `n = 12`, then its prime factors are `[2,2,3]`, then `6` and `12` are nice divisors, while `3` and `4` are not.

Return *the number of nice divisors of* `n`. Since that number can be too large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

Note that a prime number is a natural number greater than `1` that is not a product of two smaller natural numbers. The prime factors of a number `n` is a list of prime numbers such that their product equals `n`.

#### Example 1:
<pre>
<strong>Input:</strong> primeFactors = 5
<strong>Output:</strong> 6
<strong>Explanation:</strong> 200 is a valid value of n.
It has 5 prime factors: [2,2,2,5,5], and it has 6 nice divisors: [10,20,40,50,100,200].
There is not other value of n that has at most 5 prime factors and more nice divisors.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> primeFactors = 8
<strong>Output:</strong> 18
</pre>

#### Constraints:
* <code>1 <= primeFactors <= 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def maxNiceDivisors(self, primeFactors: int) -> int:
        x, y = primeFactors // 3, primeFactors % 3

        if y == 0:
            y = 1
        elif y == 1 and x > 0:
            x -= 1
            y = 4

        return pow(3, x, 1000000007) * y % 1000000007
```
