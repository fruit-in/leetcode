# 1455. Check If a Word Occurs As a Prefix of Any Word in a Sentence
Given a `sentence` that consists of some words separated by a **single space**, and a `searchWord`.

You have to check if `searchWord` is a prefix of any word in `sentence`.

Return *the index of the word* in `sentence` where `searchWord` is a prefix of this word (**1-indexed**).

If `searchWord` is a prefix of more than one word, return the index of the first word **(minimum index)**. If there is no such word return **-1**.

A **prefix** of a string `S` is any leading contiguous substring of `S`.

#### Example 1:
<pre>
<strong>Input:</strong> sentence = "i love eating burger", searchWord = "burg"
<strong>Output:</strong> 4
<strong>Explanation:</strong> "burg" is prefix of "burger" which is the 4th word in the sentence.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> sentence = "this problem is an easy problem", searchWord = "pro"
<strong>Output:</strong> 2
<strong>Explanation:</strong> "pro" is prefix of "problem" which is the 2nd and the 6th word in the sentence, but we return 2 as it's the minimal index.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> sentence = "i am tired", searchWord = "you"
<strong>Output:</strong> -1
<strong>Explanation:</strong> "you" is not a prefix of any word in the sentence.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> sentence = "i use triple pillow", searchWord = "pill"
<strong>Output:</strong> 4
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> sentence = "hello from the other side", searchWord = "they"
<strong>Output:</strong> -1
</pre>

#### Constraints:
* `1 <= sentence.length <= 100`
* `1 <= searchWord.length <= 10`
* `sentence` consists of lowercase English letters and spaces.
* `searchWord` consists of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        sentence
            .split(' ')
            .position(|s| s.starts_with(&search_word))
            .unwrap_or(-2_i32 as usize) as i32
            + 1
    }
}
```
