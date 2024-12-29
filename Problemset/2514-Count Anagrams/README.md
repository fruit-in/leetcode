# 2514. Count Anagrams
You are given a string `s` containing one or more words. Every consecutive pair of words is separated by a single space `' '`.

A string `t` is an **anagram** of string `s` if the <code>i<sup>th</sup></code> word of `t` is a **permutation** of the <code>i<sup>th</sup></code> word of `s`.

* For example, `"acb dfe"` is an anagram of `"abc def"`, but `"def cab"` and `"adc bef"` are not.

Return *the number of **distinct anagrams** of* `s`. Since the answer may be very large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> s = "too hot"
<strong>Output:</strong> 18
<strong>Explanation:</strong> Some of the anagrams of the given string are "too hot", "oot hot", "oto toh", "too toh", and "too oht".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "aa"
<strong>Output:</strong> 1
<strong>Explanation:</strong> There is only one anagram possible for the given string.
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` consists of lowercase English letters and spaces `' '`.
* There is single space between consecutive words.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_anagrams(s: String) -> i32 {
        fn power(x: i64, exp: u32) -> i64 {
            if exp == 0 {
                1
            } else if exp % 2 == 0 {
                power(x, exp / 2).pow(2) % 1_000_000_007
            } else {
                x * power(x, exp - 1) % 1_000_000_007
            }
        }

        let mut factorials = vec![1];
        let mut ret = 1_i64;

        for word in s.split_whitespace() {
            let mut count = [0; 26];
            let mut length = 0;

            for c in word.bytes() {
                count[(c - b'a') as usize] += 1;
            }

            for c in count {
                length += c;
                while length >= factorials.len() {
                    let x = (*factorials.last().unwrap() * factorials.len() as i64) % 1_000_000_007;
                    factorials.push(x);
                }
                ret = (ret * factorials[length]) % 1_000_000_007;
                ret = (ret * power(factorials[c], 1_000_000_005)) % 1_000_000_007;
                ret = (ret * power(factorials[length - c], 1_000_000_005)) % 1_000_000_007;
            }
        }

        ret as i32
    }
}
```
