# 367. Valid Perfect Square
Given a positive integer *num*, write a function which returns True if *num* is a perfect square else False.

**Note:** **Do not** use any built-in library function such as ```sqrt```.

#### Example 1:
<pre>
<strong>Input:</strong> 16
<strong>Output:</strong> true
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 14
<strong>Output:</strong> false
</pre>

## Solutions (Rust)

### 1. Brute Force
```Rust
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut i = 1;
        while i < num / i {
            i += 1;
        }
        i * i == num
    }
}
```

### 2. Binary Search
```Rust
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num == 1 {
            return true;
        }

        let mut l = 0;
        let mut r = num;
        let mut m = (l + r) / 2;

        while l <= r && m != num / m {
            if m < num / m {
                l = m + 1;
            } else if m > num / m {
                r = m - 1;
            }
            m = (l + r) / 2;
        }

        m * m == num
    }
}
```

### 3. n<sup>2</sup> = 1 + 3 + ... + (2n - 1)
```Rust
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut num = num;
        let mut i = 1;
        while num > 0 {
            num -= i;
            i += 2;
        }
        num == 0
    }
}
```

### 4. Newton's Method
```Rust
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut i = num;
        while i > num / i {
            i = (i + num / i) / 2;
        }
        i * i == num
    }
}
```
