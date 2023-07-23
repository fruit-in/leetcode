# 848. 字母移位
有一个由小写字母组成的字符串 `S`，和一个整数数组 `shifts`。

我们将字母表中的下一个字母称为原字母的 移位（由于字母表是环绕的， `'z'` 将会变成 `'a'`）。

例如，`shift('a') = 'b'`， `shift('t') = 'u'`， 以及 `shift('z') = 'a'`。

对于每个 `shifts[i] = x` ， 我们会将 `S` 中的前 `i+1` 个字母移位 `x` 次。

返回将所有这些移位都应用到 `S` 后最终得到的字符串。

#### 示例:
<pre>
<strong>输入:</strong> S = "abc", shifts = [3,5,9]
<strong>输出:</strong> "rpl"
<strong>解释:</strong>
我们以 "abc" 开始。
将 S 中的第 1 个字母移位 3 次后，我们得到 "dbc"。
再将 S 中的前 2 个字母移位 5 次后，我们得到 "igc"。
最后将 S 中的这 3 个字母移位 9 次后，我们得到答案 "rpl"。
</pre>

#### 提示:
1. `1 <= S.length = shifts.length <= 20000`
2. `0 <= shifts[i] <= 10 ^ 9`

## 题解 (Rust)

### 1. 前缀和
```Rust
impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        let mut s = s.into_bytes();
        let mut shift = 0;

        for i in (0..s.len()).rev() {
            shift = (shift + shifts[i]) % 26;
            s[i] = (s[i] - b'a' + (shift as u8)) % 26 + b'a';
        }

        String::from_utf8(s).unwrap()
    }
}
```
