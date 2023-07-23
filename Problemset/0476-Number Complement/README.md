# 476. Number Complement
Given a positive integer, output its complement number. The complement strategy is to flip the bits of its binary representation.

#### Note:
1. The given integer is guaranteed to fit within the range of a 32-bit signed integer.
2. You could assume no leading zero bit in the integer’s binary representation.

#### Example 1:
<pre>
<strong>Input:</strong> 5
<strong>Output:</strong> 2
<strong>Explanation:</strong> The binary representation of 5 is 101 (no leading zero bits), and its complement is 010. So you need to output 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 1
<strong>Output:</strong> 0
<strong>Explanation:</strong> The binary representation of 1 is 1 (no leading zero bits), and its complement is 0. So you need to output 0.
</pre>

## Solutions (Rust)

### 1. Mathematical
```Rust
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        2_i32.pow((num as f64).log2() as u32 + 1) - 1 - num
    }
}
```

### 2. Bitwise Operator
```Rust
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut ret = 0;

        for i in 0..30 {
            if num >> i == 0 {
                break;
            }
            if num & (1 << i) == 0 {
                ret ^= 1 << i;
            }
        }

        ret
    }
}
```
