# 396. Rotate Function
Given an array of integers `A` and let *n* to be its length.

Assume <code>B<sub>k</sub></code> to be an array obtained by rotating the array `A` *k* positions clock-wise, we define a "rotation function" `F` on `A` as follow:

<code>F(k) = 0 * B<sub>k</sub>[0] + 1 * B<sub>k</sub>[1] + ... + (n-1) * B<sub>k</sub>[n-1]</code>.

Calculate the maximum value of `F(0), F(1), ..., F(n-1)`.

#### Note:
*n* is guaranteed to be less than 10<sup>5</sup>.

#### Example:
```
A = [4, 3, 2, 6]

F(0) = (0 * 4) + (1 * 3) + (2 * 2) + (3 * 6) = 0 + 3 + 4 + 18 = 25
F(1) = (0 * 6) + (1 * 4) + (2 * 3) + (3 * 2) = 0 + 4 + 6 + 6 = 16
F(2) = (0 * 2) + (1 * 6) + (2 * 4) + (3 * 3) = 0 + 6 + 8 + 9 = 23
F(3) = (0 * 3) + (1 * 2) + (2 * 6) + (3 * 4) = 0 + 2 + 12 + 12 = 26

So the maximum value of F(0), F(1), F(2), F(3) is F(3) = 26.
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_rotate_function(a: Vec<i32>) -> i32 {
        let mut f = 0;
        let mut sum = 0;

        for i in 0..a.len() {
            f += i as i32 * a[i];
            sum += a[i];
        }

        let mut ret = f;

        for i in 0..a.len() {
            f += a.len() as i32 * a[i] - sum;
            ret = ret.max(f);
        }

        ret
    }
}
```
