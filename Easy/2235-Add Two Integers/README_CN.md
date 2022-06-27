# 2235. 两整数相加
给你两个整数 `num1` 和 `num2`，返回这两个整数的和。

#### 示例 1:
<pre>
<strong>输入:</strong> num1 = 12, num2 = 5
<strong>输出:</strong> 17
<strong>解释:</strong> num1 是 12，num2 是 5 ，它们的和是 12 + 5 = 17 ，因此返回 17 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num1 = -10, num2 = 4
<strong>输出:</strong> -6
<strong>解释:</strong> num1 + num2 = -6 ，因此返回 -6 。
</pre>

#### 提示:
* `-100 <= num1, num2 <= 100`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def sum(self, num1: int, num2: int) -> int:
        return num1 + num2
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn sum(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }
}
```
