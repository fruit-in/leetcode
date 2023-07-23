# 70. 爬楼梯
假设你正在爬楼梯。需要 *n* 阶你才能到达楼顶。

每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？

**注意:** 给定 *n* 是一个正整数。

#### 示例 1:
<pre>
<strong>输入:</strong> 2
<strong>输出:</strong> 2
<strong>解释:</strong> 有两种方法可以爬到楼顶。
1.  1 阶 + 1 阶
2.  2 阶
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 3
<strong>输出:</strong> 3
<strong>解释:</strong> 有三种方法可以爬到楼顶。
1.  1 阶 + 1 阶 + 1 阶
2.  1 阶 + 2 阶
3.  2 阶 + 1 阶
</pre>

## 题解 (Rust)

### 1. 递归
```Rust
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        fn helper(n: usize) -> Vec<i32> {
            match n {
                1 => vec![1],
                2 => vec![1, 2],
                _ => {
                    let mut v = helper(n - 1);
                    v.push(v[n - 2] + v[n - 3]);
                    v
                },
            }
        }
        helper(n as usize).pop().unwrap()
    }
}
```

### 2. 迭代
```Rust
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut fib = (0, 1);
        for _ in 0..n {
            fib = (fib.1, fib.0 + fib.1);
        }
        fib.1
    }
}
```
