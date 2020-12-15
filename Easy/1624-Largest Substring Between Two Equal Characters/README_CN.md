# 1624. 两个相同字符之间的最长子字符串
给你一个字符串 `s`，请你返回 **两个相同字符之间的最长子字符串的长度** ，计算长度时不含这两个字符。如果不存在这样的子字符串，返回 `-1` 。

**子字符串** 是字符串中的一个连续字符序列。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "aa"
<strong>输出:</strong> 0
<strong>解释:</strong> 最优的子字符串是两个 'a' 之间的空子字符串。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "abca"
<strong>输出:</strong> 2
<strong>解释:</strong> 最优的子字符串是 "bc" 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "cbzxy"
<strong>输出:</strong> -1
<strong>解释:</strong> s 中不存在出现出现两次的字符，所以返回 -1 。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s = "cabbac"
<strong>输出:</strong> 4
<strong>解释:</strong> 最优的子字符串是 "abba" ，其他的非最优解包括 "bb" 和 "" 。
</pre>

#### 提示:
* `1 <= s.length <= 300`
* `s` 只含小写英文字母

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {String} s
# @return {Integer}
def max_length_between_equal_characters(s)
  pairs = Array.new(26) { |_| [s.length, 0] }

  s.chars.each_with_index do |c, i|
    k = c.ord - 97

    pairs[k][0] = i if pairs[k][0] > i
    pairs[k][1] = i if pairs[k][1] < i
  end

  pairs.map { |pair| pair[1] - pair[0] - 1 }.max
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut pairs = vec![(s.len() as i32, 0); 26];

        for (i, c) in s.bytes().enumerate() {
            let pair = &mut pairs[(c - b'a') as usize];

            pair.0 = pair.0.min(i as i32);
            pair.1 = pair.1.max(i as i32);
        }

        pairs.iter().map(|(i, j)| j - i - 1).max().unwrap()
    }
}
```
