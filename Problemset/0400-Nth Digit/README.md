# 400. Nth Digit
Find the *n*<sup>th</sup> digit of the infinite integer sequence 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ...

#### Note:
*n* is positive and will fit within the range of a 32-bit signed integer (*n* < 2<sup>31</sup>).

#### Example 1:
<pre>
<strong>Input:</strong>
3
<strong>Output:</strong>
3
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
11
<strong>Output:</strong>
0
<strong>Explanation:</strong>
The 11th digit of the sequence 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ... is a 0, which is part of the number 10.
</pre>

## Solutions (Rust)

### 1. Mathematical
```Rust
impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut n = n;
        let mut a = 0;
        let mut b = 0;
        let mut c = 9;

        for i in 0..8 {
            a += 9 * (i + 1) * 10_i32.pow(i as u32);
            if a >= n {
                c = i + 1;
                break;
            }
            b = a;
        }

        n -= b + 1;
        match n % c {
            0 => 1 + n / (c * 10_i32.pow(c as u32 - 1)),
            d => n % (c * 10_i32.pow(c as u32 - d as u32)) / (c * 10_i32.pow(c as u32 - d as u32 - 1)),
        }
    }
}
```
