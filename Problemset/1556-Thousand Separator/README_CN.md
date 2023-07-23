# 1556. 千位分隔数
给你一个整数 `n`，请你每隔三位添加点（即 "." 符号）作为千位分隔符，并将结果以字符串格式返回。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 987
<strong>输出:</strong> "987"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 1234
<strong>输出:</strong> "1.234"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 123456789
<strong>输出:</strong> "123.456.789"
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> n = 0
<strong>输出:</strong> "0"
</pre>

#### 提示:
* `0 <= n < 2^31`

## 题解 (Rust)

### 1. 题解
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
