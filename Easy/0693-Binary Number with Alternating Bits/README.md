# 693. Binary Number with Alternating Bits
Given a positive integer, check whether it has alternating bits: namely, if two adjacent bits will always have different values.

#### Example 1:
<pre>
<strong>Input:</strong> 5
<strong>Output:</strong> True
<strong>Explanation:</strong>
The binary representation of 5 is: 101
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 7
<strong>Output:</strong> False
<strong>Explanation:</strong>
The binary representation of 7 is: 111.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> 11
<strong>Output:</strong> False
<strong>Explanation:</strong>
The binary representation of 11 is: 1011.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> 10
<strong>Output:</strong> True
<strong>Explanation:</strong>
The binary representation of 10 is: 1010.
</pre>

## Solutions (Rust)

### 1. Compare Last Two Bits
```Rust
impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut n = n;
        let mut pre = n & 1;
        while n != 0 {
            n >>= 1;
            if pre == n & 1 {
                return false;
            }
            pre = n & 1;
        }

        true
    }
}
```

### 2. Calculate the Valid Numbers
```Rust
impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut i = 1;
        while i > 0 && i < n {
            match i % 2 {
                1 => i = 2 * i,
                _ => i = 2 * i + 1,
            };
        }

        i == n
    }
}
```
