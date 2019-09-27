# 202. 快乐数
编写一个算法来判断一个数是不是“快乐数”。

一个“快乐数”定义为：对于一个正整数，每一次将该数替换为它每个位置上的数字的平方和，然后重复这个过程直到这个数变为 1，也可能是无限循环但始终变不到 1。如果可以变为 1，那么这个数就是快乐数。

#### 示例:
<pre>
<strong>输入:</strong> 19
<strong>输出:</strong> true
<strong>解释:</strong>
1<sup>2</sup> + 9<sup>2</sup> = 82
8<sup>2</sup> + 2<sup>2</sup> = 68
6<sup>2</sup> + 8<sup>2</sup> = 100
1<sup>2</sup> + 0<sup>2</sup> + 0<sup>2</sup> = 1
</pre>

## 题解 (Rust)

### 1. 集合
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut set = HashSet::new();
        let mut n = n;
        let mut new_n: i32;
        while !set.contains(&n) {
            set.insert(n);
            new_n = 0;
            while n > 0 {
                new_n += (n % 10).pow(2);
                n /= 10;
            }
            n = new_n;
        }
        n == 1
    }
}
```
