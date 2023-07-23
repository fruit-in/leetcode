# 1888. 使二进制字符串字符交替的最少反转次数
给你一个二进制字符串 `s` 。你可以按任意顺序执行以下两种操作任意次：

* **类型 1 ：删除** 字符串 `s` 的第一个字符并将它 **添加** 到字符串结尾。
* **类型 2 ：选择** 字符串 `s` 中任意一个字符并将该字符 **反转** ，也就是如果值为 `'0'` ，则反转得到 `'1'` ，反之亦然。

请你返回使 `s` 变成 **交替** 字符串的前提下， **类型 2** 的 **最少** 操作次数 。

我们称一个字符串是 **交替** 的，需要满足任意相邻字符都不同。

* 比方说，字符串 `"010"` 和 `"1010"` 都是交替的，但是字符串 `"0100"` 不是。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "111000"
<strong>输出:</strong> 2
<strong>解释:</strong> 执行第一种操作两次，得到 s = "100011" 。
然后对第三个和第六个字符执行第二种操作，得到 s = "101010" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "010"
<strong>输出:</strong> 0
<strong>解释:</strong> 字符串已经是交替的。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "1110"
<strong>输出:</strong> 1
<strong>解释:</strong> 对第二个字符执行第二种操作，得到 s = "1010" 。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s[i]` 要么是 `'0'` ，要么是 `'1'` 。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let mut count = 0;
        let mut ret = s.len();

        for (i, c) in s.chars().enumerate() {
            count += ((i % 2 == 0) ^ (c == '0')) as usize;
        }

        for c in s.chars() {
            if c == '0' {
                count = s.len() - count - s.len() % 2;
            } else {
                count = s.len() - count + s.len() % 2;
            }

            ret = ret.min(count).min(s.len() - count);
        }

        ret as i32
    }
}
```
