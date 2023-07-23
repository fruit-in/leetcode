# 1137. 第 N 个泰波那契数
泰波那契序列 T<sub>n</sub> 定义如下：

T<sub>0</sub> = 0, T<sub>1</sub> = 1, T<sub>2</sub> = 1, 且在 n >= 0 的条件下 T<sub>n+3</sub> = T<sub>n</sub> + T<sub>n+1</sub> + T<sub>n+2</sub>

给你整数 ```n```，请返回第 n 个泰波那契数 T<sub>n</sub> 的值。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 4
<strong>输出:</strong> 4
<strong>解释:</strong>
T_3 = 0 + 1 + 1 = 2
T_4 = 1 + 1 + 2 = 4
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 25
<strong>输出:</strong> 1389537
</pre>

#### 提示:
* ```0 <= n <= 37```
* 答案保证是一个 32 位整数，即 ```answer <= 2^31 - 1```。

## 题解 (Rust)

### 1. 递归
```Rust
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        fn helper(n: usize) -> Vec<i32> {
            match n {
                0 => vec![0],
                1 => vec![0, 1],
                2 => vec![0, 1, 1],
                _ => {
                    let mut v = helper(n - 1);
                    let t_n = v[n - 3] + v[n - 2] + v[n - 1];
                    v.push(t_n);
                    v
                },
            }
        }
        let n = n as usize;
        helper(n)[n]
    }
}
```

### 2. 迭代
```Rust
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        match n {
            0 => 0,
            1 | 2 => 1,
            _ => {
                let mut t = (0, 1, 1);
                for i in 3..=n {
                    t = (t.1, t.2, t.0 + t.1 + t.2);
                }
                t.2
            },
        }
    }
}
```
