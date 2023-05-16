# 2384. 最大回文数字
给你一个仅由数字（`0 - 9`）组成的字符串 `num` 。

请你找出能够使用 `num` 中数字形成的 **最大回文** 整数，并以字符串形式返回。该整数不含 **前导零** 。

**注意：**

* 你 **无需** 使用 `num` 中的所有数字，但你必须使用 **至少** 一个数字。
* 数字可以重新排序。

#### 示例 1:
<pre>
<strong>输入:</strong> num = "444947137"
<strong>输出:</strong> "7449447"
<strong>解释:</strong>
从 "444947137" 中选用数字 "4449477"，可以形成回文整数 "7449447" 。
可以证明 "7449447" 是能够形成的最大回文整数。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num = "00009"
<strong>输出:</strong> "9"
<strong>解释:</strong>
可以证明 "9" 能够形成的最大回文整数。
注意返回的整数不应含前导零。
</pre>

#### 提示:
* <code>1 <= num.length <= 10<sup>5</sup></code>
* `num` 由数字（`0 - 9`）组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn largest_palindromic(num: String) -> String {
        let mut count = [0; 10];
        let mut digits = vec![];

        for digit in num.bytes() {
            count[(digit - b'0') as usize] += 1;
        }

        for i in (0..=9).rev() {
            while count[i] > 1 {
                count[i] -= 2;
                digits.push(i as u8 + b'0');
            }
        }

        if *digits.get(0).unwrap_or(&b'1') == b'0' {
            digits.clear();
        }

        let mut digits_rev = digits.clone();
        digits_rev.reverse();

        if let Some(i) = (0..=9).rev().find(|&i| count[i] > 0) {
            digits.push(i as u8 + b'0');
        }
        digits.append(&mut digits_rev);
        if digits.is_empty() {
            digits.push(b'0');
        }

        String::from_utf8(digits).unwrap()
    }
}
```
