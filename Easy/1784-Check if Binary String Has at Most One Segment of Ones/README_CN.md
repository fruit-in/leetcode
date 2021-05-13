# 1784. 检查二进制字符串字段
给你一个二进制字符串 `s` ，该字符串 **不含前导零** 。

如果 `s` 最多包含 **一个由连续的 `'1'` 组成的字段** ，返回 `true`。否则，返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "1001"
<strong>输出:</strong> false
<strong>解释:</strong> 字符串中的 1 没有形成一个连续字段。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "110"
<strong>输出:</strong> true
</pre>

#### 提示:
* `1 <= s.length <= 100`
* `s[i]` 为 `'0'` 或 `'1'`
* `s[0]` 为 `'1'`

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {String} s
# @return {Boolean}
def check_ones_segment(s)
  !s.include?('01')
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        !s.contains("01")
    }
}
```
