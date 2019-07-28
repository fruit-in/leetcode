# 201. Bitwise AND of Numbers Range
Given a range [m, n] where 0 <= m <= n <= 2147483647, return the bitwise AND of all numbers in this range, inclusive.

#### Example 1:
<pre>
<strong>Input:</strong> [5,7]
<strong>Output:</strong> 4
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [0,1]
<strong>Output:</strong> 0
</pre>

#### Note:

## Solutions

### 1. Solution (Rust)
```Rust
impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let mut result = n;
        for i in m..n {
            result &= i;
        }
        result
    }
}
```

### 2. Solution (Rust)
```Rust
impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let mut n = n;
        while n > m {
            n &= n - 1;
        }
        n
    }
}
```

### 3. Solution (Rust)
```Rust
impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        if n > m {
            Self::range_bitwise_and(m >> 1, n >> 1) << 1
        } else {
            m
        }
    }
}
```

### 4. Solution (Rust)
```Rust
impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let mut result = 0;
        let mut mask = 1 << 30;
        while mask != 0 && m & mask == n & mask {
            result |= m & mask;
            mask >>= 1;
        }
        result
    }
}
```
