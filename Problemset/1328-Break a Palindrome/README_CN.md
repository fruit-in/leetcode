# 1328. 破坏回文串
给你一个回文字符串 `palindrome` ，请你将其中 一个 字符用任意小写英文字母替换，使得结果字符串的字典序最小，且 **不是** 回文串。

请你返回结果字符串。如果无法做到，则返回一个空串。

#### 示例 1:
<pre>
<strong>输入:</strong> palindrome = "abccba"
<strong>输出:</strong> "aaccba"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> palindrome = "a"
<strong>输出:</strong> ""
</pre>

#### 提示:
* `1 <= palindrome.length <= 1000`
* `palindrome` 只包含小写英文字母。

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {String} palindrome
# @return {String}
def break_palindrome(palindrome)
  return '' if palindrome.size == 1

  (0...palindrome.size / 2).each do |i|
    if palindrome[i] != 'a'
      palindrome[i] = 'a'
      return palindrome
    end
  end

  palindrome[-1] = 'b'

  palindrome
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        if palindrome.len() == 1 {
            return String::new();
        }

        let mut palindrome = palindrome.into_bytes();

        for i in 0..palindrome.len() / 2 {
            if palindrome[i] != b'a' {
                palindrome[i] = b'a';
                return String::from_utf8(palindrome).unwrap();
            }
        }

        *palindrome.last_mut().unwrap() = b'b';

        String::from_utf8(palindrome).unwrap()
    }
}
```
