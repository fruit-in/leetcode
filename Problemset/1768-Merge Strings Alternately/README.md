# 1768. Merge Strings Alternately
You are given two strings `word1` and `word2`. Merge the strings by adding letters in alternating order, starting with `word1`. If a string is longer than the other, append the additional letters onto the end of the merged string.

Return *the merged string*.

#### Example 1:
<pre>
<strong>Input:</strong> word1 = "abc", word2 = "pqr"
<strong>Output:</strong> "apbqcr"
<strong>Explanation:</strong> The merged string will be merged as so:
word1:  a   b   c
word2:    p   q   r
merged: a p b q c r
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> word1 = "ab", word2 = "pqrs"
<strong>Output:</strong> "apbqrs"
<strong>Explanation:</strong> Notice that as word2 is longer, "rs" is appended to the end.
word1:  a   b
word2:    p   q   r   s
merged: a p b q   r   s
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> word1 = "abcd", word2 = "pq"
<strong>Output:</strong> "apbqcd"
<strong>Explanation:</strong> Notice that as word1 is longer, "cd" is appended to the end.
word1:  a   b   c   d
word2:    p   q
merged: a p b q c   d
</pre>

#### Constraints:
* `1 <= word1.length, word2.length <= 100`
* `word1` and `word2` consist of lowercase English letters.

## Solutions (Rust)

### 1. Recursion
```Rust
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        if word1.is_empty() && word2.is_empty() {
            return String::new();
        }

        format!(
            "{}{}{}",
            word1.get(..1).unwrap_or(""),
            word2.get(..1).unwrap_or(""),
            Self::merge_alternately(
                word1.get(1..).unwrap_or("").to_string(),
                word2.get(1..).unwrap_or("").to_string()
            )
        )
    }
}
```
