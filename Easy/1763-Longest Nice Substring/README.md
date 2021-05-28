# 1763. Longest Nice Substring
A string `s` is **nice** if, for every letter of the alphabet that `s` contains, it appears **both** in uppercase and lowercase. For example, `"abABB"` is nice because `'A'` and `'a'` appear, and `'B'` and `'b'` appear. However, `"abA"` is not because `'b'` appears, but `'B'` does not.

Given a string `s`, return *the longest **substring** of `s` that is **nice**. If there are multiple, return the substring of the **earliest** occurrence. If there are none, return an empty string*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "YazaAay"
<strong>Output:</strong> "aAa"
<strong>Explanation:</strong> "aAa" is a nice string because 'A/a' is the only letter of the alphabet in s, and both 'A' and 'a' appear.
"aAa" is the longest nice substring.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "Bb"
<strong>Output:</strong> "Bb"
<strong>Explanation:</strong> "Bb" is a nice string because both 'B' and 'b' appear. The whole string is a substring.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "c"
<strong>Output:</strong> ""
<strong>Explanation:</strong> There are no nice substrings.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> s = "dDzeE"
<strong>Output:</strong> "dD"
<strong>Explanation:</strong> Both "dD" and "eE" are the longest nice substrings.
As there are multiple longest nice substrings, return "dD" since it occurs earlier.
</pre>

#### Constraints:
* `1 <= s.length <= 100`
* `s` consists of uppercase and lowercase English letters.

## Solutions (Ruby)

### 1. Solution
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

## Solutions (Rust)

### 1. Solution
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
