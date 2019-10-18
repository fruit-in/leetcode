# 263. 丑数
编写一个程序判断给定的数是否为丑数。

丑数就是只包含质因数 ```2, 3, 5``` 的**正整数**。

#### 示例 1:
<pre>
<strong>输入:</strong> 6
<strong>输出:</strong> true
<strong>解释:</strong> 6 = 2 × 3
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 8
<strong>输出:</strong> true
<strong>解释:</strong> 8 = 2 × 2 × 2
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> 14
<strong>输出:</strong> false
<strong>解释:</strong> 14 不是丑数，因为它包含了另外一个质因数 7。
</pre>

#### 说明:
1. ```1``` 是丑数。
2. 输入不会超过 32 位有符号整数的范围: [−2<sup>31</sup>,  2<sup>31</sup> − 1]。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn is_ugly(num: i32) -> bool {
        if num == 0 {
            false
        } else if num == 1 {
            true
        } else if num % 2 == 0 {
            Self::is_ugly(num / 2)
        } else if num % 3 == 0 {
            Self::is_ugly(num / 3)
        } else if num % 5 == 0 {
            Self::is_ugly(num / 5)
        } else {
            false
        }
    }
}
```
