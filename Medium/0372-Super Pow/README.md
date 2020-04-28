# 372. Super Pow
Your task is to calculate <i>a<sup>b</sup></i> mod 1337 where *a* is a positive integer and *b* is an extremely large positive integer given in the form of an array.

#### Example 1:
<pre>
<strong>Input:</strong> a = 2, b = [3]
<strong>Output:</strong> 8
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> a = 2, b = [1,0]
<strong>Output:</strong> 1024
</pre>

## Solutions (Rust)

### 1. Solution
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
