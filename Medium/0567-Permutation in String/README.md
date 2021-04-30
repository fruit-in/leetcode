# 567. Permutation in String
Given two strings `s1` and `s2`, return true if `s2` contains the permutation of `s1`.

In other words, one of `s1`'s permutations is the substring of `s2`.

#### Example 1:
<pre>
<strong>Input:</strong> s1 = "ab", s2 = "eidbaooo"
<strong>Output:</strong> true
<strong>Explanation:</strong> s2 contains one permutation of s1 ("ba").
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s1 = "ab", s2 = "eidboaoo"
<strong>Output:</strong> false
</pre>

#### Constraints:
* <code>1 <= s1.length, s2.length <= 10<sup>4</sup></code>
* `s1` and `s2` consist of lowercase English letters.

## Solutions (Ruby)

### 1. Sliding Window
```Ruby
# @param {String} s1
# @param {String} s2
# @return {Boolean}
def check_inclusion(s1, s2)
  return false if s1.size > s2.size

  count1 = [0] * 26
  count2 = [0] * 26

  (0...s1.size).each do |i|
    count1[s1[i].ord - 97] += 1
    count2[s2[i].ord - 97] += 1
  end

  (0..s2.size - s1.size).each do |i|
    return true if count1 == count2

    if i + s1.size < s2.size
      count2[s2[i].ord - 97] -= 1
      count2[s2[i + s1.size].ord - 97] += 1
    end
  end

  false
end
```

## Solutions (Rust)

### 1. Sliding Window
```Rust
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut count1 = [0; 26];
        let mut count2 = [0; 26];

        for i in 0..s1.len() {
            count1[(s1[i] - b'a') as usize] += 1;
            count2[(s2[i] - b'a') as usize] += 1;
        }

        for i in 0..=s2.len() - s1.len() {
            if count1 == count2 {
                return true;
            }
            if i + s1.len() < s2.len() {
                count2[(s2[i] - b'a') as usize] -= 1;
                count2[(s2[i + s1.len()] - b'a') as usize] += 1;
            }
        }

        false
    }
}
```
