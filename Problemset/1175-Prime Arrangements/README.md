# 1175. Prime Arrangements
Return the number of permutations of 1 to ```n``` so that prime numbers are at prime indices (1-indexed.)

*(Recall that an integer is prime if and only if it is greater than 1, and cannot be written as a product of two positive integers both smaller than it.)*

Since the answer may be large, return the answer **modulo ```10^9 + 7```**.

#### Example 1:
<pre>
<strong>Input:</strong> n = 5
<strong>Output:</strong> 12
<strong>Explanation:</strong> For example [1,2,5,4,3] is a valid permutation, but [5,2,3,4,1] is not because the prime number 5 is at index 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 100
<strong>Output:</strong> 682289015
</pre>

#### Constraints:
* ```1 <= n <= 100```

## Solutions (Rust)

### 1. Sieve of Eratosthenes & Permutation Number Formula
```Rust
impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        let mut is_prime = vec![true; n as usize];
        is_prime[0] = false;
        let mut primes = n - 1;

        let mut i = 2;
        while i * i <= n as usize {
            if is_prime[i - 1] {
                let mut j = i * i;
                while j <= n as usize {
                    if is_prime[j - 1] {
                        is_prime[j - 1] = false;
                        primes -= 1;
                    }
                    j += i;
                }
            }
            i += 1;
        }

        let mut ret = 1_i64;
        for i in 1..=primes {
            ret = ret * i as i64 % 1000000007;
        }
        for i in 1..=(n - primes) {
            ret = ret * i as i64 % 1000000007;
        }

        ret as i32
    }
}
```
