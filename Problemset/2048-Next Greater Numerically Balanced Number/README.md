# 2048. Next Greater Numerically Balanced Number
An integer `x` is **numerically balanced** if for every digit `d` in the number `x`, there are **exactly** `d` occurrences of that digit in `x`.

Given an integer `n`, return *the **smallest numerically balanced** number **strictly greater** than* `n`.

#### Example 1:
<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 22
<strong>Explanation:</strong>
22 is numerically balanced since:
- The digit 2 occurs 2 times.
It is also the smallest numerically balanced number strictly greater than 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 1000
<strong>Output:</strong> 1333
<strong>Explanation:</strong>
1333 is numerically balanced since:
- The digit 1 occurs 1 time.
- The digit 3 occurs 3 times.
It is also the smallest numerically balanced number strictly greater than 1000.
Note that 1022 cannot be the answer because 0 appeared more than 0 times.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 3000
<strong>Output:</strong> 3133
<strong>Explanation:</strong>
3133 is numerically balanced since:
- The digit 1 occurs 1 time.
- The digit 3 occurs 3 times.
It is also the smallest numerically balanced number strictly greater than 3000.
</pre>

#### Constraints:
* <code>0 <= n <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        let mut x = n + 1;

        loop {
            let mut count = [0; 10];
            let mut y = x as usize;

            while y > 0 {
                count[y % 10] += 1;
                y /= 10;
            }

            if (0..10).all(|i| count[i] == i || count[i] == 0) {
                return x;
            }

            x += 1;
        }

        unreachable!()
    }
}
```
