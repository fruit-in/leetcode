# 991. 坏了的计算器
在显示着数字的坏计算器上，我们可以执行以下两种操作：
* **双倍（Double）：** 将显示屏上的数字乘 2；
* **递减（Decrement）：** 将显示屏上的数字减 1 。

最初，计算器显示数字 ```X```。

返回显示数字 ```Y``` 所需的最小操作数。

#### 示例 1:
<pre>
<strong>输入:</strong> X = 2, Y = 3
<strong>输出:</strong> 2
<strong>解释:</strong> 先进行双倍运算，然后再进行递减运算 {2 -> 4 -> 3}.
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> X = 5, Y = 8
<strong>输出:</strong> 2
<strong>解释:</strong> 先递减，再双倍 {5 -> 4 -> 8}.
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> X = 3, Y = 10
<strong>输出:</strong> 3
<strong>解释:</strong> 先双倍，然后递减，再双倍 {3 -> 6 -> 5 -> 10}.
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> X = 1024, Y = 1
<strong>输出:</strong> 1023
<strong>解释:</strong> 执行递减运算 1023 次
</pre>

#### 提示:
1. ```1 <= X <= 10^9```
2. ```1 <= Y <= 10^9```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn broken_calc(x: i32, y: i32) -> i32 {
        let mut y = y;
        let mut ret = 0;

        while x < y {
            ret += 1;

            if y % 2 == 1 {
                y += 1;
            } else {
                y /= 2;
            }
        }

        ret + x - y
    }
}
```
