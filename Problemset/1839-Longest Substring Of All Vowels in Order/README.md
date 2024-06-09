# 1839. Longest Substring Of All Vowels in Order
A string is considered **beautiful** if it satisfies the following conditions:

* Each of the 5 English vowels (`'a'`, `'e'`, `'i'`, `'o'`, `'u'`) must appear **at least once** in it.
* The letters must be sorted in **alphabetical order** (i.e. all `'a'`s before `'e'`s, all `'e'`s before `'i'`s, etc.).

For example, strings `"aeiou"` and `"aaaaaaeiiiioou"` are considered **beautiful**, but `"uaeio"`, `"aeoiu"`, and `"aaaeeeooo"` are **not beautiful**.

Given a string `word` consisting of English vowels, return *the **length of the longest beautiful substring** of* `word`. *If no such substring exists, return* `0`.

A **substring** is a contiguous sequence of characters in a string.

#### Example 1:
<pre>
<strong>Input:</strong> word = "aeiaaioaaaaeiiiiouuuooaauuaeiu"
<strong>Output:</strong> 13
<strong>Explanation:</strong> The longest beautiful substring in word is "aaaaeiiiiouuu" of length 13.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> word = "aeeeiiiioooauuuaeiou"
<strong>Output:</strong> 5
<strong>Explanation:</strong> The longest beautiful substring in word is "aeiou" of length 5.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> word = "a"
<strong>Output:</strong> 0
<strong>Explanation:</strong> There is no beautiful substring, so return 0.
</pre>

#### Constraints:
* <code>1 <= word.length <= 5 * 10<sup>5</sup></code>
* `word` consists of characters `'a'`, `'e'`, `'i'`, `'o'`, and `'u'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn longest_beautiful_substring(word: String) -> i32 {
        let mut vowels = "_aeiou_".as_bytes();
        let mut i = 0;
        let mut count = 0;
        let mut ret = 0;

        for c in word.bytes().chain(std::iter::once(b' ')) {
            if c == vowels[i] {
                count += 1;
            } else if c == vowels[i + 1] {
                i += 1;
                count += 1;
            } else {
                if i == 5 {
                    ret = ret.max(count);
                }
                i = (c == b'a') as usize;
                count = i as i32;
            }
        }

        ret
    }
}
```
