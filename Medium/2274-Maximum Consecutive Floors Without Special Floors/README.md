# 2274. Maximum Consecutive Floors Without Special Floors
Alice manages a company and has rented some floors of a building as office space. Alice has decided some of these floors should be **special floors**, used for relaxation only.

You are given two integers `bottom` and `top`, which denote that Alice has rented all the floors from `bottom` to `top` (**inclusive**). You are also given the integer array `special`, where `special[i]` denotes a special floor that Alice has designated for relaxation.

Return *the **maximum** number of consecutive floors without a special floor*.

#### Example 1:
<pre>
<strong>Input:</strong> bottom = 2, top = 9, special = [4,6]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The following are the ranges (inclusive) of consecutive floors without a special floor:
- (2, 3) with a total amount of 2 floors.
- (5, 5) with a total amount of 1 floor.
- (7, 9) with a total amount of 3 floors.
Therefore, we return the maximum number which is 3 floors.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> bottom = 6, top = 8, special = [7,6,8]
<strong>Output:</strong> 0
<strong>Explanation:</strong> Every floor rented is a special floor, so we return 0.
</pre>

#### Constraints:
* <code>1 <= special.length <= 10<sup>5</sup></code>
* <code>1 <= bottom <= special[i] <= top <= 10<sup>9</sup></code>
* All the values of `special` are **unique**.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_consecutive(bottom: i32, top: i32, special: Vec<i32>) -> i32 {
        let mut special = special;

        special.push(bottom - 1);
        special.push(top + 1);
        special.sort_unstable();

        (1..special.len())
            .map(|i| special[i] - special[i - 1] - 1)
            .max()
            .unwrap()
    }
}
```
