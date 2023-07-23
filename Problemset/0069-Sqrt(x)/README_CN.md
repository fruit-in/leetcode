# 69. x 的平方根
实现 ```int sqrt(int x)``` 函数。

计算并返回 *x* 的平方根，其中 *x* 是非负整数。

由于返回类型是整数，结果只保留整数的部分，小数部分将被舍去。

#### 示例 1:
<pre>
<strong>输入:</strong> 4
<strong>输出:</strong> 2
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 8
<strong>输出:</strong> 2
<strong>说明:</strong> 8 的平方根是 2.82842..., 
     由于返回类型是整数，小数部分将被舍去。
</pre>

## 题解 (Rust)

### 1. 暴力法
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

### 2. 二分查找
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

### 4. 牛顿法
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
