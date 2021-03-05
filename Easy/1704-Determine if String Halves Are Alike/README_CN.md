# 1704. 判断字符串的两半是否相似
给你一个偶数长度的字符串 `s` 。将其拆分成长度相同的两半，前一半为 `a` ，后一半为 `b` 。

两个字符串 相似 的前提是它们都含有相同数目的元音（`'a'`，`'e'`，`'i'`，`'o'`，`'u'`，`'A'`，`'E'`，`'I'`，`'O'`，`'U'`）。注意，`s` 可能同时含有大写和小写字母。

如果 `a` 和 `b` 相似，返回 `true` ；否则，返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "book"
<strong>输出:</strong> true
<strong>解释:</strong> a = "bo" 且 b = "ok" 。a 中有 1 个元音，b 也有 1 个元音。所以，a 和 b 相似。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "textbook"
<strong>输出:</strong> false
<strong>解释:</strong> a = "text" 且 b = "book" 。a 中有 1 个元音，b 中有 2 个元音。因此，a 和 b 不相似。
注意，元音 o 在 b 中出现两次，记为 2 个。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "MerryChristmas"
<strong>输出:</strong> false
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s = "AbCdEfGh"
<strong>输出:</strong> true
</pre>

#### 提示:
* `2 <= s.length <= 1000`
* `s.length` 是偶数
* `s` 由 **大写和小写** 字母组成

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {String} s
# @return {Boolean}
def halves_are_alike(s)
  count = 0

  (0...s.size).each do |i|
    count += i < s.size / 2 ? 1 : -1 if 'aeiouAEIOU'.include?(s[i])
  end

  count == 0
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        s.bytes()
            .enumerate()
            .filter(|(_, c)| b"aeiouAEIOU".contains(c))
            .map(|(i, _)| if i < s.len() / 2 { 1 } else { -1 })
            .sum::<i32>()
            == 0
    }
}
```
