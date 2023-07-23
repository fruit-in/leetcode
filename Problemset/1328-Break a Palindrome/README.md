# 1328. Break a Palindrome
Given a palindromic string of lowercase English letters `palindrome`, replace **exactly one** character with any lowercase English letter so that the resulting string is **not** a palindrome and that it is the **lexicographically smallest** one possible.

Return *the resulting string. If there is no way to replace a character to make it not a palindrome, return an **empty string***.

A string `a` is lexicographically smaller than a string `b` (of the same length) if in the first position where `a` and `b` differ, `a` has a character strictly smaller than the corresponding character in `b`. For example, `"abcc"` is lexicographically smaller than `"abcd"` because the first position they differ is at the fourth character, and `'c'` is smaller than `'d'`.

#### Example 1:
<pre>
<strong>Input:</strong> palindrome = "abccba"
<strong>Output:</strong> "aaccba"
<strong>Explanation:</strong> There are many ways to make "abccba" not a palindrome, such as "zbccba", "aaccba", and "abacba".
Of all the ways, "aaccba" is the lexicographically smallest.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> palindrome = "a"
<strong>Output:</strong> ""
<strong>Explanation:</strong> There is no way to replace a single character to make "a" not a palindrome, so return an empty string.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> palindrome = "aa"
<strong>Output:</strong> "ab"
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> palindrome = "aba"
<strong>Output:</strong> "abb"
</pre>

#### Constraints:
* `1 <= palindrome.length <= 1000`
* `palindrome` consists of only lowercase English letters.

## Solutions (Ruby)

### 1. Solution
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

## Solutions (Rust)

### 1. Solution
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
