# 777. 在 LR 字符串中交换相邻字符
在一个由 `'L'` , `'R'` 和 `'X'` 三个字符组成的字符串（例如`"RXXLRXRXL"`）中进行移动操作。一次移动操作指用一个 `"LX"` 替换一个 `"XL"`，或者用一个 `"XR"` 替换一个 `"RX"`。现给定起始字符串 `start` 和结束字符串 `result`，请编写代码，当且仅当存在一系列移动操作使得 `start` 可以转换成 `result` 时， 返回 `True`。

#### 示例 1:
<pre>
<strong>输入:</strong> start = "RXXLRXRXL", result = "XRLXXRRLX"
<strong>输出:</strong> true
<strong>解释:</strong> 通过以下步骤我们可以将 start 转化为 result：
RXXLRXRXL ->
XRXLRXRXL ->
XRLXRXRXL ->
XRLXXRRXL ->
XRLXXRRLX
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> start = "X", result = "L"
<strong>输出:</strong> false
</pre>

#### 提示:
* <code>1 <= start.length <= 10<sup>4</sup></code>
* `start.length == result.length`
* `start` 和 `result` 都只包含 `'L'`, `'R'` 或 `'X'`。

## 题解 (Rust)

### 1. 题解
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
