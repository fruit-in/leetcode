# 372. 超级次方
你的任务是计算 <i>a<sup>b</sup></i> 对 1337 取模，*a* 是一个正整数，*b* 是一个非常大的正整数且会以数组形式给出。

#### 示例 1:
<pre>
<strong>输入:</strong> a = 2, b = [3]
<strong>输出:</strong> 8
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> a = 2, b = [1,0]
<strong>输出:</strong> 1024
</pre>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        let a = a % 1337;
        let mut b = b;
        let mut ret = 1;

        let tmp = match b.pop() {
            Some(n) => {
                for _ in 0..n {
                    ret *= a;
                    ret %= 1337;
                }
                Self::super_pow(a, b)
            },
            None => 1,
        };

        for _ in 0..10 {
            ret *= tmp;
            ret %= 1337;
        }

        ret
    }
}
```
