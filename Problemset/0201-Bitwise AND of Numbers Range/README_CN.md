# 201. 数字范围按位与
给定范围 [m, n]，其中 0 <= m <= n <= 2147483647，返回此范围内所有数字的按位与（包含 m, n 两端点）。

#### 示例 1:
<pre>
<strong>输入:</strong> [5,7]
<strong>输出:</strong> 4
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [0,1]
<strong>输出:</strong> 0
</pre>

## 题解 (Rust)

### 1. 题解1
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

### 2. 题解2
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

### 3. 题解3
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

### 4. 题解4
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
