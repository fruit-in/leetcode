# 342. 4的幂
给定一个整数 (32 位有符号整数)，请编写一个函数来判断它是否是 4 的幂次方。

#### 示例 1:
<pre>
<strong>输入:</strong> 16
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 5
<strong>输出:</strong> false
</pre>

**进阶:**
你能不使用循环或者递归来完成本题吗？

## 题解 (Rust)

### 1. 除法
```Rust
impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        let mut num = num;
        if num <= 0 {
            false
        } else {
            while num % 4 == 0 {
                num /= 4;
            }
            num == 1
        }
    }
}
```

### 2. 位操作
```Rust
impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        (num > 0) && (num & 0x55555555 == num) && (num & (num - 1) == 0)
    }
}
```
