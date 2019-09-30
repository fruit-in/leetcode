# 231. 2的幂
给定一个整数，编写一个函数来判断它是否是 2 的幂次方。

#### 示例 1:
<pre>
<strong>输入:</strong> 1
<strong>输出:</strong> true
<strong>解释:</strong> 2<sup>0</sup> = 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 16
<strong>输出:</strong> true
<strong>解释:</strong> 2<sup>4</sup> = 16
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> 218
<strong>输出:</strong> false
</pre>

## 题解 (Rust)

### 1. n & (n - 1)
```Rust
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            false
        } else {
            n & (n - 1) == 0
        }
    }
}
```

### 2. n / 2
```Rust
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            false
        } else if n == 1 {
            true
        } else {
            n % 2 == 0 && Self::is_power_of_two(n / 2)
        }
    }
}
```
