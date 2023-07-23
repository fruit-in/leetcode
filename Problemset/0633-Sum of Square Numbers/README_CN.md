# 633. 平方数之和
给定一个非负整数 ```c``` ，你要判断是否存在两个整数 ```a``` 和 ```b```，使得 a<sup>2</sup> + b<sup>2</sup> = c。

#### 示例 1:
<pre>
<strong>输入:</strong> 5
<strong>输出:</strong> True
<strong>解释:</strong> 1 * 1 + 2 * 2 = 5
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 3
<strong>输出:</strong> False
</pre>

## 题解 (Rust)

### 1. 哈希表
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

### 2. 双指针
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
