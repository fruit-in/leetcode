# 1881. Maximum Value after Insertion
You are given a very large integer `n`, represented as a string, and an integer digit `x`. The digits in `n` and the digit `x` are in the **inclusive** range `[1, 9]`, a`nd` n may represent a **negative** number.

You want to **maximize** `n`**'s numerical value** by inserting `x` anywhere in the decimal representation of `n`. You **cannot** insert `x` to the left of the negative sign.

* For example, if `n = 73` and `x = 6`, it would be best to insert it between `7` and `3`, making `n = 763`.
* If `n = -55` and `x = 2`, it would be best to insert it before the first `5`, making `n = -255`.

Return *a string representing the **maximum** value of* `n` *after the insertion*.

#### Example 1:
<pre>
<strong>Input:</strong> n = "99", x = 9
<strong>Output:</strong> "999"
<strong>Explanation:</strong> The result is the same regardless of where you insert 9.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = "-13", x = 2
<strong>Output:</strong> "-123"
<strong>Explanation:</strong> You can make n one of {-213, -123, -132}, and the largest of those three is -123.
</pre>

#### Constraints:
* <code>1 <= n.length <= 10<sup>5</sup></code>
* `1 <= x <= 9`
* The digits in `n` are in the range `[1, 9]`.
* `n` is a valid representation of an integer.
* In the case of a negative `n`, it will begin with `'-'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_value(n: String, x: i32) -> String {
        let x = x as u8 + b'0';
        let mut n = n.into_bytes();

        if n[0] != b'-' {
            for i in 0..=n.len() {
                if i == n.len() || x > n[i] {
                    n.insert(i, x);
                    break;
                }
            }
        } else {
            for i in 1..=n.len() {
                if i == n.len() || x < n[i] {
                    n.insert(i, x);
                    break;
                }
            }
        }

        String::from_utf8(n).unwrap()
    }
}
```
