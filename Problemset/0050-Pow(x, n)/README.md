# 50. Pow(x, n)
Implement <code>pow(*x*, *n*)</code>, which calculates *x* raised to the power *n* (x<sup>n</sup>).

#### Example 1:
<pre>
<strong>Input:</strong> 2.00000, 10
<strong>Output:</strong> 1024.00000
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 2.10000, 3
<strong>Output:</strong> 9.26100
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> 2.00000, -2
<strong>Output:</strong> 0.25000
<strong>Explanation:</strong> 2<sup>-2</sup> = 1/2<sup>2</sup> = 1/4 = 0.25
</pre>

#### Note:
* -100.0 < *x* < 100.0
* *n* is a 32-bit signed integer, within the range [−2<sup>31</sup>, 2<sup>31</sup> − 1]

## Solutions (Rust)

### 1. Recursion
```Rust
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n > 0 {
            let x_n2 = Self::my_pow(x, n / 2);
            if n % 2 == 0 {
                x_n2 * x_n2
            } else {
                x_n2 * x_n2 * x
            }
        } else if n < 0 {
            let x_n2 = Self::my_pow(1.0 / x, -(n / 2));
            if n % 2 == 0 {
                x_n2 * x_n2
            } else {
                x_n2 * x_n2 / x
            }
        } else {
            1.0
        }
    }
}
```
