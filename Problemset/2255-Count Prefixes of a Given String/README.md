# 2255. Count Prefixes of a Given String
You are given a string array `words` and a string `s`, where `words[i]` and `s` comprise only of **lowercase English letters**.

Return *the **number of strings** in* `words` *that are a **prefix** of* `s`.

A **prefix** of a string is a substring that occurs at the beginning of the string. A **substring** is a contiguous sequence of characters within a string.

#### Example 1:
<pre>
<strong>Input:</strong> words = ["a","b","c","ab","bc","abc"], s = "abc"
<strong>Output:</strong> 3
<strong>Explanation:</strong>
The strings in words which are a prefix of s = "abc" are:
"a", "ab", and "abc".
Thus the number of strings in words which are a prefix of s is 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> words = ["a","a"], s = "aa"
<strong>Output:</strong> 2
<strong>Explanation:</strong>
Both of the strings are a prefix of s.
Note that the same string can occur multiple times in words, and it should be counted each time.
</pre>

#### Constraints:
* `1 <= words.length <= 1000`
* `1 <= words[i].length, s.length <= 10`
* `words[i]` and `s` consist of lowercase English letters **only**.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        words
            .iter()
            .filter(|word| s.starts_with(word.as_str()))
            .count() as i32
    }
}
```
