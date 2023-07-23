# 2194. Cells in a Range on an Excel Sheet
A cell `(r, c)` of an excel sheet is represented as a string `"<col><row>"` where:
* `<col>` denotes the column number `c` of the cell. It is represented by **alphabetical letters**.
    * For example, the `1st` column is denoted by `'A'`, the `2nd` by `'B'`, the `3rd` by `'C'`, and so on.
* `<row>` is the row number `r` of the cell. The `rth` row is represented by the **integer** `r`.

You are given a string `s` in the format `"<col1><row1>:<col2><row2>"`, where `<col1>` represents the column `c1`, `<row1>` represents the row `r1`, `<col2>` represents the column `c2`, and `<row2>` represents the row `r2`, such that `r1 <= r2` and `c1 <= c2`.

Return *the **list of cells*** `(x, y)` *such that* `r1 <= x <= r2` *and* `c1 <= y <= c2`. The cells should be represented as **strings** in the format mentioned above and be sorted in **non-decreasing** order first by columns and then by rows.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/02/08/ex1drawio.png)
<pre>
<strong>Input:</strong> s = "K1:L2"
<strong>Output:</strong> ["K1","K2","L1","L2"]
<strong>Explanation:</strong>
The above diagram shows the cells which should be present in the list.
The red arrows denote the order in which the cells should be presented.
</pre>

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/02/09/exam2drawio.png)
<pre>
<strong>Input:</strong> s = "A1:F1"
<strong>Output:</strong> ["A1","B1","C1","D1","E1","F1"]
<strong>Explanation:</strong>
The above diagram shows the cells which should be present in the list.
The red arrow denotes the order in which the cells should be presented.
</pre>

#### Constraints:
* `s.length == 5`
* `'A' <= s[0] <= s[3] <= 'Z'`
* `'1' <= s[1] <= s[4] <= '9'`
* `s` consists of uppercase English letters, digits and `':'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let s = s.as_bytes();
        let mut ret = vec![];

        for col in s[0]..=s[3] {
            for row in s[1]..=s[4] {
                ret.push(String::from_utf8(vec![col, row]).unwrap());
            }
        }

        ret
    }
}
```
