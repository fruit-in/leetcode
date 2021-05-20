# 816. Ambiguous Coordinates
We had some 2-dimensional coordinates, like `"(1, 3)"` or `"(2, 0.5)"`. Then, we removed all commas, decimal points, and spaces and ended up with the string s.
* For example, `"(1, 3)"` becomes `s = "(13)"` and `"(2, 0.5)"` becomes `s = "(205)"`.

Return *a list of strings representing all possibilities for what our original coordinates could have been*.

Our original representation never had extraneous zeroes, so we never started with numbers like `"00"`, `"0.0"`, `"0.00"`, `"1.0"`, `"001"`, `"00.01"`, or any other number that can be represented with fewer digits. Also, a decimal point within a number never occurs without at least one digit occurring before it, so we never started with numbers like `".1"`.

The final answer list can be returned in any order. All coordinates in the final answer have exactly one space between them (occurring after the comma.)

#### Example 1:
<pre>
<strong>Input:</strong> s = "(123)"
<strong>Output:</strong> ["(1, 2.3)","(1, 23)","(1.2, 3)","(12, 3)"]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "(00011)"
<strong>Output:</strong> ["(0, 0.011)","(0.001, 1)"]
<strong>Explanation:</strong> 0.0, 00, 0001 or 00.01 are not allowed.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "(0123)"
<strong>Output:</strong> ["(0, 1.23)","(0, 12.3)","(0, 123)","(0.1, 2.3)","(0.1, 23)","(0.12, 3)"]
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> s = "(100)"
<strong>Output:</strong> ["(10, 0)"]
<strong>Explanation:</strong> 1.0 is not allowed.
</pre>

#### Constraints:
* `4 <= s.length <= 12`
* `s[0] == '('` and `s[s.length - 1] == ')'`.
* The rest of `s` are digits.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let mut ret = vec![];

        for i in 2..s.len() - 1 {
            for x in Self::all_possibilites(s.get(1..i).unwrap()) {
                for y in Self::all_possibilites(s.get(i..s.len() - 1).unwrap()) {
                    ret.push(format!("({}, {})", x, y));
                }
            }
        }

        ret
    }

    fn all_possibilites(s: &str) -> Vec<String> {
        let mut ret = vec![];

        if s == "0" || !s.starts_with('0') {
            ret.push(s.to_string());
        }
        for i in 1..s.len() {
            let x = format!("{}.{}", s.get(..i).unwrap(), s.get(i..).unwrap());
            if Self::is_valid(&x) {
                ret.push(x);
            }
        }

        ret
    }

    fn is_valid(s: &str) -> bool {
        let v = s.split('.').collect::<Vec<_>>();

        (v[0] == "0" || !v[0].starts_with('0')) && !v[1].ends_with('0')
    }
}
```
