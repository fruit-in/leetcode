# 313. Super Ugly Number
Write a program to find the <code>n<sup>th</sup></code> super ugly number.

Super ugly numbers are positive numbers whose all prime factors are in the given ```prime``` list primes of size ```k```.

#### Example:
<pre>
<strong>Input:</strong> n = 12, primes = [2,7,13,19]
<strong>Output:</strong> 32
<strong>Explanation:</strong> [1,2,4,7,8,13,14,16,19,26,28,32] is the sequence of the first 12 
             super ugly numbers given primes = [2,7,13,19] of size 4.
</pre>

#### Note:
* ```1``` is a super ugly number for any given ```primes```.
* The given numbers in ```primes``` are in ascending order.
* 0 < ```k``` ≤ 100, 0 < ```n``` ≤ 10<sup>6</sup>, 0 < ```primes[i]``` < 1000.
* The n<sup>th</sup> super ugly number is guaranteed to fit in a 32-bit signed integer.

## Solutions (Rust)

### 1. Dynamic Programming
```Rust
impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let mut uglys = vec![1];
        let mut points = vec![0; primes.len()];

        for _ in 1..n {
            let mut min_ugly = primes[0] * uglys[points[0]];
            for i in 1..points.len() {
                min_ugly = min_ugly.min(primes[i] * uglys[points[i]]);
            }

            uglys.push(min_ugly);

            for i in 0..points.len() {
                if primes[i] * uglys[points[i]] == min_ugly {
                    points[i] += 1;
                }
            }
        }

        uglys[n as usize - 1]
    }
}
```
