# 777. Swap Adjacent in LR String
In a string composed of `'L'`, `'R'`, and `'X'` characters, like `"RXXLRXRXL"`, a move consists of either replacing one occurrence of `"XL"` with `"LX"`, or replacing one occurrence of `"RX"` with `"XR"`. Given the starting string `start` and the ending string `result`, return `True` if and only if there exists a sequence of moves to transform `start` to `result`.

#### Example 1:
<pre>
<strong>Input:</strong> start = "RXXLRXRXL", result = "XRLXXRRLX"
<strong>Output:</strong> true
<strong>Explanation:</strong> We can transform start to result following these steps:
RXXLRXRXL ->
XRXLRXRXL ->
XRLXRXRXL ->
XRLXXRRXL ->
XRLXXRRLX
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> start = "X", result = "L"
<strong>Output:</strong> false
</pre>

#### Constraints:
* <code>1 <= start.length <= 10<sup>4</sup></code>
* `start.length == result.length`
* Both `start` and `result` will only consist of characters in `'L'`, `'R'`, and `'X'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn can_transform(start: String, result: String) -> bool {
        let start = start.as_bytes();
        let result = result.as_bytes();
        let mut i = 0;
        let mut j = 0;

        loop {
            while i < start.len() && start[i] == b'X' {
                i += 1;
            }
            while j < result.len() && result[j] == b'X' {
                j += 1;
            }

            if i == start.len() || j == result.len() {
                return i == start.len() && j == result.len();
            } else if (start[i] == b'L' && result[j] == b'L' && i >= j)
                || (start[i] == b'R' && result[j] == b'R' && i <= j)
            {
                i += 1;
                j += 1;
            } else {
                return false;
            }
        }

        unreachable!()
    }
}
```
