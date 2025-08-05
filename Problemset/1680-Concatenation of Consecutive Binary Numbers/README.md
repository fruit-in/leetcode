# 1680. Concatenation of Consecutive Binary Numbers
Given an integer `n`, return *the **decimal value** of the binary string formed by concatenating the binary representations of* `1` *to* `n` *in order, **modulo*** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> "1" in binary corresponds to the decimal value 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 3
<strong>Output:</strong> 27
<strong>Explanation:</strong> In binary, 1, 2, and 3 corresponds to "1", "10", and "11".
After concatenating them, we have "11011", which corresponds to the decimal value 27.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 12
<strong>Output:</strong> 505379714
<strong>Explanation:</strong> The concatenation results in "1101110010111011110001001101010111100".
The decimal value of that is 118505380540.
After modulo 10<sup>9</sup> + 7, the result is 505379714.
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        fn pow_of_2(exp: u32) -> i64 {
            if exp == 0 {
                1
            } else if exp % 2 == 0 {
                pow_of_2(exp / 2) * pow_of_2(exp / 2) % 1_000_000_007
            } else {
                pow_of_2(exp / 2) * pow_of_2(exp / 2) * 2 % 1_000_000_007
            }
        }

        let mut shl = 0;
        let mut ret = 0;

        for x in (1..=n as i64).rev() {
            ret = (ret + x * pow_of_2(shl)) % 1_000_000_007;
            shl += x.ilog2() + 1;
        }

        ret as i32
    }
}
```
