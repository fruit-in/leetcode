# 1888. Minimum Number of Flips to Make the Binary String Alternating
You are given a binary string `s`. You are allowed to perform two types of operations on the string in any sequence:

* **Type-1: Remove** the character at the start of the string `s` and **append** it to the end of the string.
* **Type-2: Pick** any character in `s` and **flip** its value, i.e., if its value is `'0'` it becomes `'1'` and vice-versa.

Return *the **minimum** number of **type-2** operations you need to perform such that* `s` *becomes **alternating***.

The string is called **alternating** if no two adjacent characters are equal.

* For example, the strings `"010"` and `"1010"` are alternating, while the string `"0100"` is not.

#### Example 1:
<pre>
<strong>Input:</strong> s = "111000"
<strong>Output:</strong> 2
<strong>Explanation:</strong> Use the first operation two times to make s = "100011".
Then, use the second operation on the third and sixth elements to make s = "101010".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "010"
<strong>Output:</strong> 0
<strong>Explanation:</strong> The string is already alternating.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "1110"
<strong>Output:</strong> 1
<strong>Explanation:</strong> Use the second operation on the second element to make s = "1010".
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s[i]` is either `'0'` or `'1'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let mut count = 0;
        let mut ret = s.len();

        for (i, c) in s.chars().enumerate() {
            count += ((i % 2 == 0) ^ (c == '0')) as usize;
        }

        for c in s.chars() {
            if c == '0' {
                count = s.len() - count - s.len() % 2;
            } else {
                count = s.len() - count + s.len() % 2;
            }

            ret = ret.min(count).min(s.len() - count);
        }

        ret as i32
    }
}
```
