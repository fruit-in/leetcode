# 880. 索引处的解码字符串
给定一个编码字符串 `S`。请你找出 **解码字符串** 并将其写入磁带。解码时，从编码字符串中 **每次读取一个字符** ，并采取以下步骤：

* 如果所读的字符是字母，则将该字母写在磁带上。
* 如果所读的字符是数字（例如 `d`），则整个当前磁带总共会被重复写 `d-1` 次。

现在，对于给定的编码字符串 `S` 和索引 `K`，查找并返回解码字符串中的第 `K` 个字母。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "leet2code3", k = 10
<strong>输出:</strong> "o"
<strong>解释:</strong> 解码后的字符串为 "leetleetcodeleetleetcodeleetleetcode"。
字符串中的第 10 个字母是 "o"。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "ha22", k = 5
<strong>输出:</strong> "h"
<strong>解释:</strong> 解码后的字符串为 "hahahaha"。第 5 个字母是 "h"。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "a2345678999999999999999", k = 1
<strong>输出:</strong> "a"
<strong>解释:</strong> 解码后的字符串为 "a" 重复 8301530446056247680 次。第 1 个字母是 "a"。
</pre>

#### 提示:
* `2 <= S.length <= 100`
* `S` 只包含小写字母与数字 `2` 到 `9` 。
* `S` 以字母开头。
* <code>1 <= k <= 10<sup>9</sup></code>
* 题目保证 `K` 小于或等于解码字符串的长度。
* 解码后的字符串保证少于 <code>2<sup>63</sup></code> 个字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut k = k as i64 - 1;
        let mut chars = vec![];
        let mut length = 0;

        for ch in s.bytes() {
            chars.push((ch, length));

            if ch.is_ascii_lowercase() {
                length += 1;
            } else {
                length *= (ch - b'0') as i64;
            }

            if length > k {
                break;
            }
        }

        while let Some((ch, i)) = chars.pop() {
            if ch.is_ascii_lowercase() {
                if i == k {
                    return String::from_utf8(vec![ch]).unwrap();
                }

                length -= 1;
            } else {
                length /= (ch - b'0') as i64;
                k %= length;
            }
        }

        unreachable!()
    }
}
```
