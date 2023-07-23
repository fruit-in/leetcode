# 279. 完全平方数
给定正整数 *n*，找到若干个完全平方数（比如```1, 4, 9, 16, ...```）使得它们的和等于 *n*。你需要让组成和的完全平方数的个数最少。

#### 示例 1:
<pre>
<strong>输入:</strong> <em>n</em> = 12
<strong>输出:</strong> 3
<strong>解释:</strong> 12 = 4 + 4 + 4
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> <em>n</em> = 13
<strong>输出:</strong> 2
<strong>解释:</strong> 13 = 4 + 9
</pre>

## 题解 (Rust)

### 1. 广度优先搜索
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let pt_sq: Vec<i32> = (1..=n).map(|x| x * x).filter(|&x| x <= n).collect();
        let mut set = HashSet::new();
        let mut v = vec![n];
        let mut cnt = 1;
 
        while !v.is_empty() {
            for _ in 0..v.len() {
                let num = v.remove(0);
                for i in &pt_sq {
                    if num - i == 0 {
                        return cnt;
                    } else if num - i > 0 && !set.contains(&(num - i)) {
                        set.insert(num - i);
                        v.push(num - i);
                    } else if num - i < 0 {
                        break;
                    }
                }
            }
            cnt += 1;
        }
        0
    }
}
```

### 2. 动态规划
```Rust
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut v = vec![0; n];
 
        let mut i = 1;
        while i * i <= n {
            v[i * i - 1] = 1;
            i += 1;
        }
 
        for i in 2..=n {
            if v[i - 1] == 0 {
                v[i - 1] = i as i32;
                let mut j = 1;
                while j * j < i {
                    v[i - 1] = v[i - 1].min(v[i - j * j - 1] + 1);
                    j += 1;
                }
            }
        }
        v[n - 1]
    }
}
```
