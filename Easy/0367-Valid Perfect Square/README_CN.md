# 367. 有效的完全平方数
给定一个正整数 *num*，编写一个函数，如果 *num* 是一个完全平方数，则返回 True，否则返回 False。

**说明:** 不要使用任何内置的库函数，如 ```sqrt```。

#### 示例 1:
<pre>
<strong>输入:</strong> 16
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 14
<strong>输出:</strong> false
</pre>

## 题解 (Rust)

### 1. 暴力法
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

### 2. 二分查找
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

### 4. 牛顿法
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
