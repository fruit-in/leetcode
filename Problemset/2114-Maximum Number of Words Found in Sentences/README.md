# 2114. Maximum Number of Words Found in Sentences
A **sentence** is a list of **words** that are separated by a single space with no leading or trailing spaces.

You are given an array of strings `sentences`, where each `sentences[i]` represents a single **sentence**.

Return *the **maximum number of words** that appear in a single sentence*.

#### Example 1:
<pre>
<strong>Input:</strong> sentences = ["alice and bob love leetcode", "i think so too", "this is great thanks very much"]
<strong>Output:</strong> 6
<strong>Explanation:</strong>
- The first sentence, "alice and bob love leetcode", has 5 words in total.
- The second sentence, "i think so too", has 4 words in total.
- The third sentence, "this is great thanks very much", has 6 words in total.
Thus, the maximum number of words in a single sentence comes from the third sentence, which has 6 words.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> sentences = ["please wait", "continue to fight", "continue to win"]
<strong>Output:</strong> 3
<strong>Explanation:</strong> It is possible that multiple sentences contain the same number of words.
In this example, the second and third sentences (underlined) have the same number of words.
</pre>

#### Constraints:
* `1 <= sentences.length <= 100`
* `1 <= sentences[i].length <= 100`
* `sentences[i]` consists only of lowercase English letters and `' '` only.
* `sentences[i]` does not have leading or trailing spaces.
* All the words in `sentences[i]` are separated by a single space.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        sentences
            .iter()
            .map(|s| s.chars().filter(|c| c.is_whitespace()).count())
            .max()
            .unwrap() as i32
            + 1
    }
}
```
