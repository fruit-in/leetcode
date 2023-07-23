# 393. UTF-8 编码验证
UTF-8 中的一个字符可能的长度为 **1 到 4 字节**，遵循以下的规则：
1. 对于 1 字节的字符，字节的第一位设为 0 ，后面 7 位为这个符号的 unicode 码。
2. 对于 n 字节的字符 (n > 1)，第一个字节的前 n 位都设为1，第 n+1 位设为 0 ，后面字节的前两位一律设为 10 。剩下的没有提及的二进制位，全部为这个符号的 unicode 码。

这是 UTF-8 编码的工作方式：
```
   Char. number range  |        UTF-8 octet sequence
      (hexadecimal)    |              (binary)
   --------------------+---------------------------------------------
   0000 0000-0000 007F | 0xxxxxxx
   0000 0080-0000 07FF | 110xxxxx 10xxxxxx
   0000 0800-0000 FFFF | 1110xxxx 10xxxxxx 10xxxxxx
   0001 0000-0010 FFFF | 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
```

给定一个表示数据的整数数组，返回它是否为有效的 utf-8 编码。

**注意:**
输入是整数数组。只有每个整数的 **最低 8 个有效位** 用来存储数据。这意味着每个整数只表示 1 字节的数据。

#### 示例 1:
<pre>
data = [197, 130, 1], 表示 8 位的序列: <strong>11000101 10000010 00000001</strong>.

返回 <strong>true</strong> 。
这是有效的 utf-8 编码，为一个2字节字符，跟着一个1字节字符。
</pre>

#### 示例 2:
<pre>
data = [235, 140, 4], 表示 8 位的序列: <strong>11101011 10001100 00000100</strong>.

返回 <strong>false</strong> 。
前 3 位都是 1 ，第 4 位为 0 表示它是一个3字节字符。
下一个字节是开头为 10 的延续字节，这是正确的。
但第二个延续字节不以 10 开头，所以是不符合规则的。
</pre>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut data = data.into_iter();

        while let Some(byte) = data.next() {
            match Self::leading_ones(byte as u8) {
                0 => (),
                n if n > 1 && n < 5 => {
                    for _ in 1..n {
                        if Self::leading_ones(data.next().unwrap_or(0) as u8) != 1 {
                            return false;
                        }
                    }
                }
                _ => return false,
            }
        }

        true
    }

    fn leading_ones(n: u8) -> u32 {
        for i in 0..8 {
            if n & (128 >> i) == 0 {
                return i;
            }
        }

        8
    }
}
```
