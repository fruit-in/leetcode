# 816. 模糊坐标
我们有一些二维坐标，如 `"(1, 3)"` 或 `"(2, 0.5)"`，然后我们移除所有逗号，小数点和空格，得到一个字符串`S`。返回所有可能的原始字符串到一个列表中。

原始的坐标表示法不会存在多余的零，所以不会出现类似于"00", "0.0", "0.00", "1.0", "001", "00.01"或一些其他更小的数来表示坐标。此外，一个小数点前至少存在一个数，所以也不会出现“.1”形式的数字。

最后返回的列表可以是任意顺序的。而且注意返回的两个数字中间（逗号之后）都有一个空格。

#### 示例 1:
<pre>
<strong>输入:</strong> "(123)"
<strong>输出:</strong> ["(1, 2.3)","(1, 23)","(1.2, 3)","(12, 3)"]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "(00011)"
<strong>输出:</strong> ["(0, 0.011)","(0.001, 1)"]
<strong>解释:</strong> 0.0, 00, 0001 或 00.01 是不被允许的。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> "(0123)"
<strong>输出:</strong> ["(0, 1.23)","(0, 12.3)","(0, 123)","(0.1, 2.3)","(0.1, 23)","(0.12, 3)"]
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> "(100)"
<strong>输出:</strong> ["(10, 0)"]
<strong>解释:</strong> 1.0 是不被允许的。
</pre>

#### 提示:
* `4 <= S.length <= 12`
* `S[0]` = "(", `S[S.length - 1]` = ")", 且字符串 `S` 中的其他元素都是数字。

## 题解 (Rust)

### 1. 题解
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
