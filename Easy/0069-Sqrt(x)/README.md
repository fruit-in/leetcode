# 69. Sqrt(x)
Implement <code>int sqrt(int x)</code>.

Compute and return the square root of x, where x is guaranteed to be a non-negative integer.

Since the return type is an integer, the decimal digits are truncated and only the integer part of the result is returned.

#### Example 1:
<pre>
<strong>Input:</strong> 4
<strong>Output:</strong> 2
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 8
<strong>Output:</strong> 2
<strong>Explanation:</strong> The square root of 8 is 2.82842..., and since the decimal part is truncated, 2 is returned.
</pre>

## Solutions (Rust)

### 1. Brute Force
```Rust
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut n = 1;
        while n <= x / n {
            n += 1;
        }
        n - 1
    }
}
```

### 2. Binary Search
```Rust
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 || x == 1 {
            return x;
        }
        let mut left = 0;
        let mut right = x;
        let mut mid = (left + right) / 2;
        loop {
            if mid <= x / mid && (mid + 1) > x / (mid + 1) {
                return mid;
            } else if mid > x / mid {
                right = mid;
            } else if mid < x / mid {
                left = mid;
            }
            mid = (left + right) / 2;
        }
    }
}
```

### 3. (n + 1)<sup>2</sup> = n<sup>2</sup> + 2n + 1
```Rust
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut n = 0;
        let mut x = x - 1;
        while x >= 0 {
            n += 1;
            x -= 2 * n + 1;
        }
        n
    }
}
```

### 4. Newton's Method
```Rust
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let x = x as usize;
        let mut n = x;
        while n > x / n {
            n = (n + x / n) / 2;
        }
        n as i32
    }
}
```
