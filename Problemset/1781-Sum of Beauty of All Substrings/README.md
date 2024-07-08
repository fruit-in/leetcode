# 1781. Sum of Beauty of All Substrings
The **beauty** of a string is the difference in frequencies between the most frequent and least frequent characters.

* For example, the beauty of `"abaacc"` is `3 - 1 = 2`.

Given a string `s`, return *the sum of **beauty** of all of its substrings*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "aabcb"
<strong>Output:</strong> 5
<strong>Explanation:</strong> The substrings with non-zero beauty are ["aab","aabc","aabcb","abcb","bcb"], each with beauty equal to 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "aabcbaa"
<strong>Output:</strong> 17
</pre>

#### Constraints:
* `1 <= s.length <= 500`
* `s` consists of only lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn beauty_sum(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ret = 0;

        for i in 0..s.len() {
            let mut count = [0; 26];

            for j in i..s.len() {
                count[(s[j] - b'a') as usize] += 1;

                let max_freq = count.iter().max().unwrap();
                let min_freq = count.iter().filter(|&&x| x > 0).min().unwrap();

                ret += max_freq - min_freq;
            }
        }

        ret
    }
}
```
