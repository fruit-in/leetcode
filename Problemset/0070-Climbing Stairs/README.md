# 70. Climbing Stairs
You are climbing a stair case. It takes *n* steps to reach to the top.

Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

**Note:** Given *n* will be a positive integer.

#### Example 1:
<pre>
<strong>Input:</strong> 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are two ways to climb to the top.
1. 1 step + 1 step
2. 2 steps
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> There are three ways to climb to the top.
1. 1 step + 1 step + 1 step
2. 1 step + 2 steps
3. 2 steps + 1 step
</pre>

## Solutions (Rust)

### 1. Recursion
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

### 2. Iteration
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
