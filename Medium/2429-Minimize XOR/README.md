# 2429. Minimize XOR
Given two positive integers `num1` and `num2`, find the positive integer `x` such that:

* `x` has the same number of set bits as `num2`, and
* The value `x XOR num1` is **minimal**.

Note that `XOR` is the bitwise XOR operation.

Return *the integer* `x`. The test cases are generated such that `x` is **uniquely determined**.

The number of **set bits** of an integer is the number of `1`'s in its binary representation.

#### Example 1:
<pre>
<strong>Input:</strong> num1 = 3, num2 = 5
<strong>Output:</strong> 3
<strong>Explanation:</strong>
The binary representations of num1 and num2 are 0011 and 0101, respectively.
The integer 3 has the same number of set bits as num2, and the value 3 XOR 3 = 0 is minimal.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num1 = 1, num2 = 12
<strong>Output:</strong> 3
<strong>Explanation:</strong>
The binary representations of num1 and num2 are 0001 and 1100, respectively.
The integer 3 has the same number of set bits as num2, and the value 3 XOR 1 = 2 is minimal.
</pre>

#### Constraints:
* <code>1 <= num1, num2 <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let ones1 = num1.count_ones() as i32;
        let ones2 = num2.count_ones() as i32;
        let mut i = 0;
        let mut x = num1;

        for _ in 0..(ones1 - ones2).max(ones2 - ones1) {
            if ones1 > ones2 {
                while x & (1 << i) == 0 {
                    i += 1;
                }

                x ^= 1 << i;
            } else {
                while x & (1 << i) != 0 {
                    i += 1;
                }

                x |= 1 << i;
            }
        }

        x
    }
}
```
