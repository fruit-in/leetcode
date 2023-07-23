# 1137. N-th Tribonacci Number
The Tribonacci sequence T<sub>n</sub> is defined as follows:

T<sub>0</sub> = 0, T<sub>1</sub> = 1, T<sub>2</sub> = 1, and T<sub>n+3</sub> = T<sub>n</sub> + T<sub>n+1</sub> + T<sub>n+2</sub> for n >= 0.

Given <code>n</code>, return the value of T<sub>n</sub>.

#### Example 1:
<pre>
<strong>Input:</strong> n = 4
<strong>Output:</strong> 4
<strong>Explanation:</strong>
T_3 = 0 + 1 + 1 = 2
T_4 = 1 + 1 + 2 = 4
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 25
<strong>Output:</strong> 1389537
</pre>

#### Constraints:
* <code>0 <= n <= 37</code>
* The answer is guaranteed to fit within a 32-bit integer, ie. <code>answer <= 2<sup>31</sup> - 1</code>.

## Solutions (Rust)

### 1. Recursion
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

### 2. Iteration
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
