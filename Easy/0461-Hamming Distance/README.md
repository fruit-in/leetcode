# 461. Hamming Distance
The Hamming distance between two integers is the number of positions at which the corresponding bits are different.

Given two integers <code>x</code> and <code>y</code>, calculate the Hamming distance.

**Note:** 0 ≤ <code>x</code>, <code>y</code> < 2<sup>31</sup>.

#### Example 1:
<pre>
<strong>Input:</strong> x = 1, y = 4
<strong>Output:</strong> 2
<strong>Explanation:</strong>
1   (0 0 0 1)
4   (0 1 0 0)
       ↑   ↑

The above arrows point to positions where the corresponding bits are different.
</pre>

## Solutions (Rust)

### 1. XOR
```Rust
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut counter = 0;
        let mut z = x ^ y;
        while z != 0 {
            counter += z & 1;
            z >>= 1;
        }
        counter
    }
}
```
