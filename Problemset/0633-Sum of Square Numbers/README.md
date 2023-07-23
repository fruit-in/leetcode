# 633. Sum of Square Numbers
Given a non-negative integer ```c```, your task is to decide whether there're two integers ```a``` and ```b``` such that a<sup>2</sup> + b<sup>2</sup> = c.

#### Example 1:
<pre>
<strong>Input:</strong> 5
<strong>Output:</strong> True
<strong>Explanation:</strong> 1 * 1 + 2 * 2 = 5
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 3
<strong>Output:</strong> False
</pre>

## Solutions (Rust)

### 1. HashMap
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut b2 = HashSet::new();
        let mut a = 0_i32;

        while let Some(a2) = a.checked_mul(a) {
            if a2 > c {
                break;
            }

            b2.insert(a2);

            if b2.contains(&(c - a2)) {
                return true;
            }

            a += 1;
        }

        false
    }
}
```

### 2. Two Pointers
```Rust
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut a = 0;
        let mut b = c.min(46340);

        while a <= b {
            if c - b * b > a * a {
                a += 1;
            } else if c - b * b < a * a {
                b -= 1;
            } else {
                return true;
            }
        }

        false
    }
}
```
