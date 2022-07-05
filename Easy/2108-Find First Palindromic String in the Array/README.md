# 2108. Find First Palindromic String in the Array
Given an array of strings `words`, return *the first **palindromic** string in the array*. If there is no such string, return *an **empty string*** `""`.

A string is **palindromic** if it reads the same forward and backward.

#### Example 1:
<pre>
<strong>Input:</strong> words = ["abc","car","ada","racecar","cool"]
<strong>Output:</strong> "ada"
<strong>Explanation:</strong> The first string that is palindromic is "ada".
Note that "racecar" is also palindromic, but it is not the first.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> words = ["notapalindrome","racecar"]
<strong>Output:</strong> "racecar"
<strong>Explanation:</strong> The first and only string that is palindromic is "racecar".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> words = ["def","ghi"]
<strong>Output:</strong> ""
<strong>Explanation:</strong> There are no palindromic strings, so the empty string is returned.
</pre>

#### Constraints:
* `1 <= words.length <= 100`
* `1 <= words[i].length <= 100`
* `words[i]` consists only of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        words
            .into_iter()
            .find(|w| {
                let v = w.as_bytes();
                (0..v.len() / 2).all(|i| v[i] == v[v.len() - 1 - i])
            })
            .unwrap_or("".to_string())
    }
}
```
