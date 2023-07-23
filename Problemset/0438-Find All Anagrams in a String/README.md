# 438. Find All Anagrams in a String
Given two strings `s` and `p`, return *an array of all the start indices of* `p`*'s anagrams in* `s`. You may return the answer in **any order**.

#### Example 1:
<pre>
<strong>Input:</strong> s = "cbaebabacd", p = "abc"
<strong>Output:</strong> [0,6]
<strong>Explanation:</strong>
The substring with start index = 0 is "cba", which is an anagram of "abc".
The substring with start index = 6 is "bac", which is an anagram of "abc".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "abab", p = "ab"
<strong>Output:</strong> [0,1,2]
<strong>Explanation:</strong>
The substring with start index = 0 is "ab", which is an anagram of "ab".
The substring with start index = 1 is "ba", which is an anagram of "ab".
The substring with start index = 2 is "ab", which is an anagram of "ab".
</pre>

#### Constraints:
* <code>1 <= s.length, p.length <= 3 * 10<sup>4</sup></code>
* `s` and `p` consist of lowercase English letters.

## Solutions (Ruby)

### 1. Sliding Window
```Ruby
# @param {String} s
# @param {String} p
# @return {Integer[]}
def find_anagrams(s, p)
  return [] if s.size < p.size

  count_p = [0] * 26
  count_s = [0] * 26
  ret = []

  (0...p.size).each do |i|
    count_p[p[i].ord - 97] += 1
    count_s[s[i].ord - 97] += 1
  end

  (0..s.size - p.size).each do |i|
    ret.push(i) if count_p == count_s
    if i + p.size < s.size
      count_s[s[i].ord - 97] -= 1
      count_s[s[i + p.size].ord - 97] += 1
    end
  end

  ret
end
```

## Solutions (Rust)

### 1. Sliding Window
```Rust
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() {
            return vec![];
        }

        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut count_p = [0; 26];
        let mut count_s = [0; 26];
        let mut ret = vec![];

        for i in 0..p.len() {
            count_p[(p[i] - b'a') as usize] += 1;
            count_s[(s[i] - b'a') as usize] += 1;
        }

        for i in 0..=s.len() - p.len() {
            if count_p == count_s {
                ret.push(i as i32);
            }
            if i + p.len() < s.len() {
                count_s[(s[i] - b'a') as usize] -= 1;
                count_s[(s[i + p.len()] - b'a') as usize] += 1;
            }
        }

        ret
    }
}
```
