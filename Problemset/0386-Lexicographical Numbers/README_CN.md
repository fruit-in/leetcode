# 386. 字典序排数
给定一个整数 *n*, 返回从 *1* 到 *n* 的字典顺序。

例如，

给定 *n* =13，返回 [1,10,11,12,13,2,3,4,5,6,7,8,9] 。

请尽可能的优化算法的时间复杂度和空间复杂度。 输入的数据 *n* 小于等于 5,000,000。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut x = 1;
        let mut ret = vec![];

        for _ in 0..n {
            ret.push(x);

            if x * 10 <= n {
                x *= 10;
            } else if x % 10 == 9 || x == n {
                x /= 10;
                while x % 10 == 9 {
                    x /= 10;
                }
                x += 1;
            } else {
                x += 1;
            }
        }

        ret
    }
}
```
