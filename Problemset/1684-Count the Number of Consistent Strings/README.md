# 1684. Count the Number of Consistent Strings
You are given a string `allowed` consisting of **distinct** characters and an array of strings `words`. A string is **consistent** if all characters in the string appear in the string `allowed`.

Return *the number of **consistent** strings in the array* `words`.

#### Example 1:
<pre>
<strong>Input:</strong> allowed = "ab", words = ["ad","bd","aaab","baa","badab"]
<strong>Output:</strong> 2
<strong>Explanation:</strong> Strings "aaab" and "baa" are consistent since they only contain characters 'a' and 'b'.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> allowed = "abc", words = ["a","b","c","ab","ac","bc","abc"]
<strong>Output:</strong> 7
<strong>Explanation:</strong> All strings are consistent.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> allowed = "cad", words = ["cc","acd","b","ba","bac","bad","ac","d"]
<strong>Output:</strong> 4
<strong>Explanation:</strong> Strings "cc", "acd", "ac", and "d" are consistent.
</pre>

#### Constraints:
* <code>1 <= words.length <= 10<sup>4</sup></code>
* `1 <= allowed.length <= 26`
* `1 <= words[i].length <= 10`
* The characters in `allowed` are **distinct**.
* `words[i]` and `allowed` contain only lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let allowed = allowed.bytes().fold(0, |ac, c| ac | (1 << (c - b'a')));

        words
            .iter()
            .filter(|word| word.bytes().all(|c| allowed & (1 << (c - b'a')) != 0))
            .count() as i32
    }
}
```
