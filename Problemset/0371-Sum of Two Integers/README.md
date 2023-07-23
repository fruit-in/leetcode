# 371. Sum of Two Integers
Calculate the sum of two integers *a* and *b*, but you are **not allowed** to use the operator ```+``` and ```-```.

#### Example 1:
<pre>
<strong>Input:</strong> a = 1, b = 2
<strong>Output:</strong> 3
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> a = -2, b = 3
<strong>Output:</strong> 1
</pre>

## Solutions (Rust)

### 1. Bitwise Operator
```Rust
impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut ans = 0;
        let mut mask = 1;
        let mut c = false;
        for _ in 0..32 {
            if (a & mask == b & mask && c) || (a & mask != b & mask && !c) {
                ans |= mask;
            }
            if a & mask != 0 && b & mask != 0 {
                c = true;
            }
            if a & mask == 0 && b & mask == 0 {
                c = false;
            }
            mask <<= 1;
        }
        ans
    }
}
```
