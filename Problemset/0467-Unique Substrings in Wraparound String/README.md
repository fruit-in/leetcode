# 467. Unique Substrings in Wraparound String
We define the string `s` to be the infinite wraparound string of `"abcdefghijklmnopqrstuvwxyz"`, so `s` will look like this:
* `"...zabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcd...."`.

Given a string `p`, return *the number of **unique non-empty substrings** of* `p` *are present in* `s`.

#### Example 1:
<pre>
<strong>Input:</strong> p = "a"
<strong>Output:</strong> 1
<strong>Explanation:</strong> Only the substring "a" of p is in s.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> p = "cac"
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are two substrings ("a", "c") of p in s.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> p = "zab"
<strong>Output:</strong> 6
<strong>Explanation:</strong> There are six substrings ("z", "a", "b", "za", "ab", and "zab") of p in s.
</pre>

#### Constraints:
* <code>1 <= p.length <= 10<sup>5</sup></code>
* `p` consists of lowercase English letters.

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {String} p
# @return {Integer}
def find_substring_in_wrapround_string(p)
  p = p.bytes
  count = 1
  max_len = [0] * 26

  (0...p.size).each do |i|
    count = i == 0 || (p[i] + 26 - p[i - 1]) % 26 != 1 ? 1 : count + 1
    (0...[26, count].min).each do |j|
      k = p[i + 1 - count + j] - 97
      max_len[k] = [max_len[k], count - j].max
    end
  end

  max_len.sum
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        let p = p.as_bytes();
        let mut count = 1;
        let mut max_len = [0; 26];

        for i in 0..p.len() {
            if i == 0 || (p[i] + 26 - p[i - 1]) % 26 != 1 {
                count = 1;
            } else {
                count += 1;
            }
            for j in 0..26.min(count) {
                let k = (p[i + 1 - count + j] - b'a') as usize;
                max_len[k] = max_len[k].max(count - j);
            }
        }

        max_len.iter().sum::<usize>() as i32
    }
}
```
