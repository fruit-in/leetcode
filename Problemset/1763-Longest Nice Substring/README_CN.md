# 1763. 最长的美好子字符串
当一个字符串 `s` 包含的每一种字母的大写和小写形式 **同时** 出现在 `s` 中，就称这个字符串 `s` 是 **美好** 字符串。比方说，`"abABB"` 是美好字符串，因为 `'A'` 和 `'a'` 同时出现了，且 `'B'` 和 `'b'` 也同时出现了。然而，`"abA"` 不是美好字符串因为 `'b'` 出现了，而 `'B'` 没有出现。

给你一个字符串 `s` ，请你返回 `s` 最长的 **美好子字符串** 。如果有多个答案，请你返回 **最早** 出现的一个。如果不存在美好子字符串，请你返回一个空字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "YazaAay"
<strong>输出:</strong> "aAa"
<strong>解释:</strong> "aAa" 是一个美好字符串，因为这个子串中仅含一种字母，其小写形式 'a' 和大写形式 'A' 也同时出现了。
"aAa" 是最长的美好子字符串。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "Bb"
<strong>输出:</strong> "Bb"
<strong>解释:</strong> "Bb" 是美好字符串，因为 'B' 和 'b' 都出现了。整个字符串也是原字符串的子字符串。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "c"
<strong>输出:</strong> ""
<strong>解释:</strong> 没有美好子字符串。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s = "dDzeE"
<strong>输出:</strong> "dD"
<strong>解释:</strong> "dD" 和 "eE" 都是最长美好子字符串。
由于有多个美好子字符串，返回 "dD" ，因为它出现得最早。
</pre>

#### 提示:
* `1 <= s.length <= 100`
* `s` 只包含大写和小写英文字母。

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {String} s
# @return {String}
def longest_nice_substring(s)
  ret = ''

  (0...s.size).each do |i|
    counter = [0] * 26

    (i...s.size).each do |j|
      if ('A'..'Z').include?(s[j])
        counter[s[j].ord - 65] |= 1
      else
        counter[s[j].ord - 97] |= 2
      end
      ret = s[i..j] if counter.all? { |c| c % 3 == 0 } && j - i + 1 > ret.size
    end
  end

  ret
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        let s = s.as_bytes();
        let mut ret = "";

        for i in 0..s.len() {
            let mut counter = [0; 26];

            for j in i..s.len() {
                match s[j] {
                    b'A'..=b'Z' => counter[(s[j] - b'A') as usize] |= 1,
                    _ => counter[(s[j] - b'a') as usize] |= 2,
                }
                if counter.iter().all(|&c| c % 3 == 0) && j - i + 1 > ret.len() {
                    ret = std::str::from_utf8(&s[i..=j]).unwrap();
                }
            }
        }

        ret.to_string()
    }
}
```
