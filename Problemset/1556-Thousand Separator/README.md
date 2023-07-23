# 1556. Thousand Separator
Given an integer `n`, add a dot (".") as the thousands separator and return it in string format.

#### Example 1:
<pre>
<strong>Input:</strong> n = 987
<strong>Output:</strong> "987"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 1234
<strong>Output:</strong> "1.234"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 123456789
<strong>Output:</strong> "123.456.789"
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> n = 0
<strong>Output:</strong> "0"
</pre>

#### Constraints:
* `0 <= n < 2^31`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        let s = n.to_string();
        let v = s.split("").filter(|&c| c != "").collect::<Vec<_>>();
        let v = v.rchunks(3).rev().map(|c| c.concat()).collect::<Vec<_>>();

        v.join(".")
    }
}
```
