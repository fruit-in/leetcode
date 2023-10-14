# 2595. Number of Even and Odd Bits
You are given a **positive** integer `n`.

Let `even` denote the number of even indices in the binary representation of `n` (**0-indexed**) with value `1`.

Let `odd` denote the number of odd indices in the binary representation of `n` (**0-indexed**) with value `1`.

Return *an integer array* `answer` *where* `answer = [even, odd]`.

#### Example 1:
<pre>
<strong>Input:</strong> n = 17
<strong>Output:</strong> [2,0]
<strong>Explanation:</strong> The binary representation of 17 is 10001.
It contains 1 on the 0th and 4th indices.
There are 2 even and 0 odd indices.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> [0,1]
<strong>Explanation:</strong> The binary representation of 2 is 10.
It contains 1 on the 1st index.
There are 0 even and 1 odd indices.
</pre>

#### Constraints:
* `1 <= n <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn even_odd_bit(n: i32) -> Vec<i32> {
        vec![
            (n & 0x555).count_ones() as i32,
            (n & 0xaaa).count_ones() as i32,
        ]
    }
}
```
