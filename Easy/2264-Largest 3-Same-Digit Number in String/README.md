# 2264. Largest 3-Same-Digit Number in String
You are given a string `num` representing a large integer. An integer is **good** if it meets the following conditions:
* It is a **substring** of `num` with length `3`.
* It consists of only one unique digit.

Return *the **maximum good** integer as a **string** or an empty string* `""` *if no such integer exists*.

Note:
* A **substring** is a contiguous sequence of characters within a string.
* There may be **leading zeroes** in `num` or a good integer.

#### Example 1:
<pre>
<strong>Input:</strong> num = "6777133339"
<strong>Output:</strong> "777"
<strong>Explanation:</strong> There are two distinct good integers: "777" and "333".
"777" is the largest, so we return "777".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num = "2300019"
<strong>Output:</strong> "000"
<strong>Explanation:</strong> "000" is the only good integer.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> num = "42352338"
<strong>Output:</strong> ""
<strong>Explanation:</strong> No substring of length 3 consists of only one unique digit. Therefore, there are no good integers.
</pre>

#### Constraints:
* `3 <= num.length <= 1000`
* `num` only consists of digits.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let num = num.into_bytes();
        let goods = num.windows(3).filter(|n| n[0] == n[1] && n[1] == n[2]);
        let largest_good = goods.max().unwrap_or(&[]);

        String::from_utf8(largest_good.to_vec()).unwrap()
    }
}
```
