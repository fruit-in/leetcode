# 1704. Determine if String Halves Are Alike
You are given a string `s` of even length. Split this string into two halves of equal lengths, and let `a` be the first half and `b` be the second half.

Two strings are **alike** if they have the same number of vowels (`'a'`, `'e'`, `'i'`, `'o'`, `'u'`, `'A'`, `'E'`, `'I'`, `'O'`, `'U'`). Notice that `s` contains uppercase and lowercase letters.

Return `true` *if* `a` *and* `b` *are **alike***. Otherwise, return `false`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "book"
<strong>Output:</strong> true
<strong>Explanation:</strong> a = "bo" and b = "ok". a has 1 vowel and b has 1 vowel. Therefore, they are alike.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "textbook"
<strong>Output:</strong> false
<strong>Explanation:</strong> a = "text" and b = "book". a has 1 vowel whereas b has 2. Therefore, they are not alike.
Notice that the vowel o is counted twice.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "MerryChristmas"
<strong>Output:</strong> false
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> s = "AbCdEfGh"
<strong>Output:</strong> true
</pre>

#### Constraints:
* `2 <= s.length <= 1000`
* `s.length` is even.
* `s` consists of **uppercase and lowercase** letters.

## Solutions (Ruby)

### 1. Solution
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

## Solutions (Rust)

### 1. Solution
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
