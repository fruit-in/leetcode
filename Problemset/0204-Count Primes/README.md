# 204. Count Primes
Count the number of prime numbers less than a non-negative number, ***n***.

#### Example:
<pre>
<strong>Input:</strong> 10
<strong>Output:</strong> 4
<strong>Explanation:</strong> There are 4 prime numbers less than 10, they are 2, 3, 5, 7.
</pre>

## Solutions (Rust)

### 1. Brute Force
```Rust
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut cnt = 0;
        for i in 2..n {
            let mut j = 2;
            while j * j <= i && i % j != 0 {
                j += 1;
            }
            if j * j > i {
                cnt += 1;
            }
        }
        cnt
    }
}
```

### 2. Sieve of Eratosthenes
```Rust
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 3 {
            return 0;
        }
        let mut primes = n - 2;
        let mut is_prime = vec![true; n as usize];
        let mut i = 2;
        while i * i < n as usize {
            if is_prime[i] {
                let mut j = i * i;
                while j < n as usize {
                    if is_prime[j] {
                        is_prime[j] = false;
                        primes -= 1;
                    }
                    j += i;
                }
            }
            i += 1;
        }
        primes
    }
}
```
