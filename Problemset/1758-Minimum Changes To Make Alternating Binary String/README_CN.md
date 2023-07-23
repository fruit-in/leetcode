# 1758. 生成交替二进制字符串的最少操作数
给你一个仅由字符 `'0'` 和 `'1'` 组成的字符串 `s` 。一步操作中，你可以将任一 `'0'` 变成 `'1'` ，或者将 `'1'` 变成 `'0'` 。

**交替字符串** 定义为：如果字符串中不存在相邻两个字符相等的情况，那么该字符串就是交替字符串。例如，字符串 `"010"` 是交替字符串，而字符串 `"0100"` 不是。

返回使 `s` 变成 **交替字符串** 所需的 **最少** 操作数。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "0100"
<strong>输出:</strong> 1
<strong>解释:</strong> 如果将最后一个字符变为 '1' ，s 就变成 "0101" ，即符合交替字符串定义。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "10"
<strong>输出:</strong> 0
<strong>解释:</strong> s 已经是交替字符串。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "1111"
<strong>输出:</strong> 2
<strong>解释:</strong> 需要 2 步操作得到 "0101" 或 "1010" 。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>4</sup></code>
* `s[i]` 是 `'0'` 或 `'1'`

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {String} s
# @return {Integer}
def min_operations(s)
  b = 0
  count = 0

  s.each_byte do |c|
    count += 1 if c != b + 48
    b = 1 - b
  end

  [count, s.size - count].min
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut b = b'0';
        let mut count = 0;

        for c in s.bytes() {
            if c != b {
                count += 1;
            }
            b = b'1' - b + b'0';
        }

        count.min(s.len() - count) as i32
    }
}
```
