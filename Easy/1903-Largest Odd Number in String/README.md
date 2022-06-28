# 1903. Largest Odd Number in String
You are given a string `num`, representing a large integer. Return *the **largest-valued odd** integer (as a string) that is a **non-empty substring** of* `num`, *or an empty string* `""` *if no odd integer exists*.

A **substring** is a contiguous sequence of characters within a string.

#### Example 1:
<pre>
<strong>Input:</strong> num = "52"
<strong>Output:</strong> "5"
<strong>Explanation:</strong> The only non-empty substrings are "5", "2", and "52". "5" is the only odd number.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num = "4206"
<strong>Output:</strong> ""
<strong>Explanation:</strong> There are no odd numbers in "4206".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> num = "35427"
<strong>Output:</strong> "35427"
<strong>Explanation:</strong> "35427" is already an odd number.
</pre>

#### Constraints:
* <code>1 <= num.length <= 10<sup>5</sup></code>
* `num` only consists of digits and does not contain any leading zeros.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let mut num = num.into_bytes();

        while *num.last().unwrap_or(&1) % 2 == 0 {
            num.pop();
        }

        String::from_utf8(num).unwrap()
    }
}
```
