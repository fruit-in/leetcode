# 509. 斐波那契数
**斐波那契数**，通常用 ```F(n)``` 表示，形成的序列称为**斐波那契数列**。该数列由 ```0``` 和 ```1``` 开始，后面的每一项数字都是前面两项数字的和。也就是：
```
F(0) = 0,   F(1) = 1
F(N) = F(N - 1) + F(N - 2), 其中 N > 1.
```

给定 ```N```，计算 ```F(N)```。

#### 示例 1:
<pre>
<strong>输入:</strong> 2
<strong>输出:</strong> 1
<strong>解释:</strong> F(2) = F(1) + F(0) = 1 + 0 = 1.
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 3
<strong>输出:</strong> 2
<strong>解释:</strong> F(3) = F(2) + F(1) = 1 + 1 = 2.
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> 4
<strong>输出:</strong> 3
<strong>解释:</strong> F(4) = F(3) + F(2) = 2 + 1 = 3.
</pre>

#### 提示:
* 0 ≤ ```N``` ≤ 30

## 题解 (Rust)

### 1. 递归
```Rust
impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 || n == 1 {
            n
        } else {
            Self::fib(n - 1) + Self::fib(n - 2)
        }
    }
}
```

### 2. 迭代
```Rust
impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return n;
        }
        let mut pre1 = 1;
        let mut pre2 = 0;
        let mut fib_num = 1;
        for i in 2..=n {
            fib_num = pre1 + pre2;
            pre2 = pre1;
            pre1 = fib_num;
        }
        fib_num
    }
}
```
