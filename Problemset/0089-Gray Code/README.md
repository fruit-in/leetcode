# 89. Gray Code
The gray code is a binary numeral system where two successive values differ in only one bit.

Given a non-negative integer *n* representing the total number of bits in the code, print the sequence of gray code. A gray code sequence must begin with 0.

#### Example 1:
<pre>
<strong>Input:</strong> 2
<strong>Output:</strong> [0,1,3,2]
<strong>Explanation:</strong>
00 - 0
01 - 1
11 - 3
10 - 2

For a given <em>n</em>, a gray code sequence may not be uniquely defined.
For example, [0,2,3,1] is also a valid gray code sequence.

00 - 0
10 - 2
11 - 3
01 - 1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 0
<strong>Output:</strong> [0]
<strong>Explanation:</strong> We define the gray code sequence to begin with 0.
             A gray code sequence of <em>n</em> has size = 2<sup>n</sup>, which for <em>n</em> = 0 the size is 2<sup>0</sup> = 1.
             Therefore, for <em>n</em> = 0 the gray code sequence is [0].
</pre>

## Solutions (Rust)

### 1. Mathematical
```Rust
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut x = 1;
        let mut ret = vec![0];

        for _ in 0..n {
            let mut rev = ret.iter().rev().map(|&num| num + x).collect();
            ret.append(&mut rev);
            x *= 2;
        }

        ret
    }
}
```
