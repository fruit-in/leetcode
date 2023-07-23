# 1813. Sentence Similarity III
A sentence is a list of words that are separated by a single space with no leading or trailing spaces. For example, `"Hello World"`, `"HELLO"`, `"hello world hello world"` are all sentences. Words consist of **only** uppercase and lowercase English letters.

Two sentences `sentence1` and `sentence2` are **similar** if it is possible to insert an arbitrary sentence (**possibly empty**) inside one of these sentences such that the two sentences become equal. For example, `sentence1 = "Hello my name is Jane"` and `sentence2 = "Hello Jane"` can be made equal by inserting `"my name is"` between `"Hello"` and `"Jane"` in `sentence2`.

Given two sentences `sentence1` and `sentence2`, return `true` *if* `sentence1` *and* `sentence2` *are similar*. Otherwise, return `false`.

#### Example 1:
<pre>
<strong>Input:</strong> sentence1 = "My name is Haley", sentence2 = "My Haley"
<strong>Output:</strong> true
<strong>Explanation:</strong> sentence2 can be turned to sentence1 by inserting "name is" between "My" and "Haley".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> sentence1 = "of", sentence2 = "A lot of words"
<strong>Output:</strong> false
<strong>Explanation:</strong> No single sentence can be inserted inside one of the sentences to make it equal to the other.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> sentence1 = "Eating right now", sentence2 = "Eating"
<strong>Output:</strong> true
<strong>Explanation:</strong> sentence2 can be turned to sentence1 by inserting "right now" at the end of the sentence.
</pre>

#### Constraints:
* `1 <= sentence1.length, sentence2.length <= 100`
* `sentence1` and `sentence2` consist of lowercase and uppercase English letters and spaces.
* The words in `sentence1` and `sentence2` are separated by a single space.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let mut i = 0;
        let mut j = 0;
        let mut words1 = sentence1.split_whitespace().collect::<Vec<_>>();
        let mut words2 = sentence2.split_whitespace().collect::<Vec<_>>();
        if words1.len() > words2.len() {
            let temp = words1;
            words1 = words2;
            words2 = temp;
        }

        while i < words1.len() && words1[i] == words2[i] {
            i += 1;
        }
        while j < words1.len() && words1[words1.len() - 1 - j] == words2[words2.len() - 1 - j] {
            j += 1;
        }

        i + j >= words1.len()
    }
}
```
