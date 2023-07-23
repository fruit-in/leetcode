# 866. Prime Palindrome
Find the smallest prime palindrome greater than or equal to `N`.

Recall that a number is *prime* if it's only divisors are 1 and itself, and it is greater than 1.

For example, 2,3,5,7,11 and 13 are primes.

Recall that a number is a *palindrome* if it reads the same from left to right as it does from right to left.

For example, 12321 is a palindrome.

#### Example 1:
<pre>
<strong>Input:</strong> 6
<strong>Output:</strong> 7
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 8
<strong>Output:</strong> 11
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> 13
<strong>Output:</strong> 101
</pre>

#### Note:
* `1 <= N <= 10^8`
* The answer is guaranteed to exist and be less than `2 * 10^8`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn prime_palindrome(mut n: i32) -> i32 {
        loop {
            match n {
                999 | 99_999 | 9_999_999 => n = n * 10 + 11,
                n if Self::is_palindrome(n) && Self::is_prime(n) => return n,
                _ => n += 1,
            }
        }
    }

    fn is_palindrome(mut n: i32) -> bool {
        if n % 10 == 0 {
            return false;
        }

        let mut rev = 0;

        while n > rev {
            rev *= 10;
            rev += n % 10;
            n /= 10;
        }

        n == rev || n == rev / 10
    }

    fn is_prime(n: i32) -> bool {
        let mut i = 2;

        while i * i <= n && n % i != 0 {
            i += 1;
        }

        n > 1 && i * i > n
    }
}
```
