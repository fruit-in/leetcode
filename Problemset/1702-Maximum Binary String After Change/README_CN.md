# 1702. 修改后的最大二进制字符串
给你一个二进制字符串 `binary` ，它仅有 `0` 或者 `1` 组成。你可以使用下面的操作任意次对它进行修改：
* 操作 1 ：如果二进制串包含子字符串 `"00"` ，你可以用 `"10"` 将其替换。
    * 比方说， `"00010" -> "10010"`
* 操作 2 ：如果二进制串包含子字符串 `"10"` ，你可以用 `"01"` 将其替换。
    * 比方说， `"00010" -> "00001"`

请你返回执行上述操作任意次以后能得到的 **最大二进制字符串** 。如果二进制字符串 `x` 对应的十进制数字大于二进制字符串 `y` 对应的十进制数字，那么我们称二进制字符串 `x` 大于二进制字符串 `y` 。

#### 示例 1:
<pre>
<strong>输入:</strong> binary = "000110"
<strong>输出:</strong> "111011"
<strong>解释:</strong> 一个可行的转换为：
"000110" -> "000101"
"000101" -> "100101"
"100101" -> "110101"
"110101" -> "110011"
"110011" -> "111011"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> binary = "01"
<strong>输出:</strong> "01"
<strong>解释:</strong> "01" 没办法进行任何转换。
</pre>

#### 提示:
* <code>1 <= binary.length <= 10<sup>5</sup></code>
* `binary` 仅包含 `'0'` 和 `'1'` 。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn maximum_binary_string(binary: String) -> String {
        let count0 = binary.bytes().filter(|&b| b == b'0').count();
        let count1 = binary.bytes().take_while(|&b| b == b'1').count();
        let mut binary = binary.into_bytes();

        if count0 > 1 {
            binary = vec![b'1'; binary.len()];
            binary[count0 + count1 - 1] = b'0';
        }

        String::from_utf8(binary).unwrap()
    }
}
```
