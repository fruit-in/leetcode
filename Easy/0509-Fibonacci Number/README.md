# 509. Fibonacci Number
The **Fibonacci numbers**, commonly denoted <code>F(n)</code> form a sequence, called the **Fibonacci sequence**, such that each number is the sum of the two preceding ones, starting from <code>0</code> and <code>1</code>. That is,

```
F(0) = 0,   F(1) = 1
F(N) = F(N - 1) + F(N - 2), for N > 1.
```

Given <code>N</code>, calculate <code>F(N)</code>.

#### Example 1:
<pre>
<strong>Input:</strong> 2
<strong>Output:</strong> 1
<strong>Explanation:</strong> F(2) = F(1) + F(0) = 1 + 0 = 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> F(3) = F(2) + F(1) = 1 + 1 = 2.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> 4
<strong>Output:</strong> 3
<strong>Explanation:</strong> F(4) = F(3) + F(2) = 2 + 1 = 3.
</pre>

#### Note:
0 ≤ <code>N</code> ≤ 30.

## Solutions

### 1. Recursion (Rust)
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

### 2. Iteration (Rust)
```Rust
impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 || n == 1{
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
