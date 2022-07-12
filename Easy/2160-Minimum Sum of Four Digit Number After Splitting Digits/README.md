# 2160. Minimum Sum of Four Digit Number After Splitting Digits
You are given a **positive** integer `num` consisting of exactly four digits. Split `num` into two new integers `new1` and `new2` by using the **digits** found in `num`. **Leading zeros** are allowed in `new1` and `new2`, and **all** the digits found in `num` must be used.

* For example, given `num = 2932`, you have the following digits: two `2`'s, one `9` and one `3`. Some of the possible pairs `[new1, new2]` are `[22, 93]`, `[23, 92]`, `[223, 9]` and `[2, 329]`.

Return *the **minimum** possible sum of* `new1` *and* `new2`.

#### Example 1:
<pre>
<strong>Input:</strong> num = 2932
<strong>Output:</strong> 52
<strong>Explanation:</strong> Some possible pairs [new1, new2] are [29, 23], [223, 9], etc.
The minimum sum can be obtained by the pair [29, 23]: 29 + 23 = 52.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num = 4009
<strong>Output:</strong> 13
<strong>Explanation:</strong> Some possible pairs [new1, new2] are [0, 49], [490, 0], etc.
The minimum sum can be obtained by the pair [4, 9]: 4 + 9 = 13.
</pre>

#### Constraints:
* `1000 <= num <= 9999`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let mut digits = [num / 1000, num / 100 % 10, num / 10 % 10, num % 10];
        digits.sort_unstable();

        digits[0] * 10 + digits[2] + digits[1] * 10 + digits[3]
    }
}
```
