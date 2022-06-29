# 2103. Rings and Rods
There are `n` rings and each ring is either red, green, or blue. The rings are distributed **across ten rods** labeled from `0` to `9`.

You are given a string `rings` of length `2n` that describes the `n` rings that are placed onto the rods. Every two characters in `rings` forms a **color-position pair** that is used to describe each ring where:
* The **first** character of the <code>i<sup>th</sub></code> pair denotes the <code>i<sup>th</sub></code> ring's **color** (`'R'`, `'G'`, `'B'`).
* The **second** character of the <code>i<sup>th</sub></code> pair denotes the **rod** that the <code>i<sup>th</sub></code> ring is placed on (`'0'` to `'9'`).

For example, `"R3G2B1"` describes `n == 3` rings: a red ring placed onto the rod labeled 3, a green ring placed onto the rod labeled 2, and a blue ring placed onto the rod labeled 1.

Return *the number of rods that have **all three colors** of rings on them*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/11/23/ex1final.png)
<pre>
<strong>Input:</strong> rings = "B0B6G0R6R0R6G9"
<strong>Output:</strong> 1
<strong>Explanation:</strong>
- The rod labeled 0 holds 3 rings with all colors: red, green, and blue.
- The rod labeled 6 holds 3 rings, but it only has red and blue.
- The rod labeled 9 holds only a green ring.
Thus, the number of rods with all three colors is 1.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/11/23/ex2final.png)
<pre>
<strong>Input:</strong> rings = "B0R0G0R9R0B0G0"
<strong>Output:</strong> 1
<strong>Explanation:</strong>
- The rod labeled 0 holds 6 rings with all colors: red, green, and blue.
- The rod labeled 9 holds only a red ring.
Thus, the number of rods with all three colors is 1.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> rings = "G4"
<strong>Output:</strong> 0
<strong>Explanation:</strong>
Only one ring is given. Thus, no rods have all three colors.
</pre>

#### Constraints:
* `rings.length == 2 * n`
* `1 <= n <= 100`
* `rings[i]` where `i` is **even** is either `'R'`, `'G'`, or `'B'` (**0-indexed**).
* `rings[i]` where `i` is **odd** is a digit from `'0'` to `'9'` (**0-indexed**).

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let rings = rings.as_bytes();
        let mut rods = [0; 10];

        for i in (0..rings.len()).step_by(2) {
            match (rings[i], (rings[i + 1] - b'0') as usize) {
                (b'R', r) => rods[r] |= 1,
                (b'G', r) => rods[r] |= 2,
                (_, r) => rods[r] |= 4,
            }
        }

        rods.iter().filter(|&&r| r == 7).count() as i32
    }
}
```
