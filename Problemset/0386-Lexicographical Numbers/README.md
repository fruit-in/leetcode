# 386. Lexicographical Numbers
Given an integer *n*, return 1 - *n* in lexicographical order.

For example, given 13, return: [1,10,11,12,13,2,3,4,5,6,7,8,9].

Please optimize your algorithm to use less time and space. The input size may be as large as 5,000,000.

## Solutions (Rust)

### 1. Solution
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
