# 204. 计数质数
统计所有小于非负整数 *n* 的质数的数量。

#### 示例:
<pre>
<strong>输入:</strong> 10
<strong>输出:</strong> 4
<strong>解释:</strong> 小于 10 的质数一共有 4 个, 它们是 2, 3, 5, 7 。
</pre>

## 题解 (Rust)

### 1. 暴力法
```Rust
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut cnt = 0;
        for i in 2..n {
            let mut j = 2;
            while j * j <= i && i % j != 0 {
                j += 1;
            }
            if j * j > i {
                cnt += 1;
            }
        }
        cnt
    }
}
```

### 2. 厄拉多塞筛法
```Rust
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 3 {
            return 0;
        }
        let mut primes = n - 2;
        let mut is_prime = vec![true; n as usize];
        let mut i = 2;
        while i * i < n as usize {
            if is_prime[i] {
                let mut j = i * i;
                while j < n as usize {
                    if is_prime[j] {
                        is_prime[j] = false;
                        primes -= 1;
                    }
                    j += i;
                }
            }
            i += 1;
        }
        primes
    }
}
```
