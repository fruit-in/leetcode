# 1963. Minimum Number of Swaps to Make the String Balanced
You are given a **0-indexed** string `s` of **even** length `n`. The string consists of **exactly** `n / 2` opening brackets `'['` and `n / 2` closing brackets `']'`.

A string is called **balanced** if and only if:

* It is the empty string, or
* It can be written as `AB`, where both `A` and `B` are **balanced** strings, or
* It can be written as `[C]`, where `C` is a **balanced** string.

You may swap the brackets at **any** two indices **any** number of times.

Return *the **minimum** number of swaps to make* `s` ***balanced***.

#### Example 1:
<pre>
<strong>Input:</strong> s = "][]["
<strong>Output:</strong> 1
<strong>Explanation:</strong> You can make the string balanced by swapping index 0 with index 3.
The resulting string is "[[]]".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "]]][[["
<strong>Output:</strong> 2
<strong>Explanation:</strong> You can do the following to make the string balanced:
- Swap index 0 with index 4. s = "[]][][".
- Swap index 1 with index 5. s = "[[][]]".
The resulting string is "[[][]]".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "[]"
<strong>Output:</strong> 0
<strong>Explanation:</strong> The string is already balanced.
</pre>

#### Constraints:
* `n == s.length`
* <code>2 <= n <= 10<sup>6</sup></code>
* `n` is even.
* `s[i]` is either `'['` or `']'`.
* The number of opening brackets `'['` equals `n / 2`, and the number of closing brackets `']'` equals `n / 2`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut count = 0;
        let mut ret = 0;

        for c in s.chars() {
            if c == '[' {
                count += 1;
            } else if c == ']' && count > 0 {
                count -= 1;
            } else {
                count += 1;
                ret += 1;
            }
        }

        ret
    }
}
```
