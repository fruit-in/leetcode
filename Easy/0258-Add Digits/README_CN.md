# 258. 各位相加
给定一个非负整数 ```num```，反复将各个位上的数字相加，直到结果为一位数。

#### 示例:
<pre>
<strong>输入:</strong> 38
<strong>输出:</strong> 2
<strong>解释:</strong> 各位相加的过程为：3 + 8 = 11, 1 + 1 = 2。 由于 2 是一位数，所以返回 2。
</pre>

#### 进阶:
你可以不使用循环或者递归，且在 O(1) 时间复杂度内解决这个问题吗？

## 题解 (Rust)

### 1. 迭代
```Rust
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut num = num;
        while num >= 10 {
            let mut tmp = 0;
            while num != 0 {
                tmp += num % 10;
                num /= 10;
            }
            num = tmp;
        }
        num
    }
}
```

### 2. 取余
```Rust
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        (num - 1) % 9 + 1
    }
}
```
