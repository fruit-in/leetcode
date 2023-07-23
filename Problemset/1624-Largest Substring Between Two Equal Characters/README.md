# 1624. Largest Substring Between Two Equal Characters
Given a string `s`, return *the length of the longest substring between two equal characters, excluding the two characters*. If there is no such substring return `-1`.

A **substring** is a contiguous sequence of characters within a string.

#### Example 1:
<pre>
<strong>Input:</strong> s = "aa"
<strong>Output:</strong> 0
<strong>Explanation:</strong> The optimal substring here is an empty substring between the two 'a's.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "abca"
<strong>Output:</strong> 2
<strong>Explanation:</strong> The optimal substring here is "bc".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "cbzxy"
<strong>Output:</strong> -1
<strong>Explanation:</strong> There are no characters that appear twice in s.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> s = "cabbac"
<strong>Output:</strong> 4
<strong>Explanation:</strong> The optimal substring here is "abba". Other non-optimal substrings include "bb" and "".
</pre>

#### Constraints:
* `1 <= s.length <= 300`
* `s` contains only lowercase English letters.

## Solutions (Ruby)

### 1. Solution
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

## Solutions (Rust)

### 1. Solution
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
