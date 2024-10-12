# 2452. Words Within Two Edits of Dictionary
You are given two string arrays, `queries` and `dictionary`. All words in each array comprise of lowercase English letters and have the same length.

In one **edit** you can take a word from `queries`, and change any letter in it to any other letter. Find all words from `queries` that, after a **maximum** of two edits, equal some word from `dictionary`.

Return *a list of all words from* `queries`*, that match with some word from* `dictionary` *after a maximum of **two edits***. Return the words in the **same order** they appear in `queries`.

#### Example 1:
<pre>
<strong>Input:</strong> queries = ["word","note","ants","wood"], dictionary = ["wood","joke","moat"]
<strong>Output:</strong> ["word","note","wood"]
<strong>Explanation:</strong>
- Changing the 'r' in "word" to 'o' allows it to equal the dictionary word "wood".
- Changing the 'n' to 'j' and the 't' to 'k' in "note" changes it to "joke".
- It would take more than 2 edits for "ants" to equal a dictionary word.
- "wood" can remain unchanged (0 edits) and match the corresponding dictionary word.
Thus, we return ["word","note","wood"].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> queries = ["yes"], dictionary = ["not"]
<strong>Output:</strong> []
<strong>Explanation:</strong>
Applying any two edits to "yes" cannot make it equal to "not". Thus, we return an empty array.
</pre>

#### Constraints:
* `1 <= queries.length, dictionary.length <= 100`
* `n == queries[i].length == dictionary[j].length`
* `1 <= n <= 100`
* All `queries[i]` and `dictionary[j]` are composed of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        let mut ret = vec![];

        for wordq in &queries {
            for wordd in &dictionary {
                if wordq
                    .chars()
                    .zip(wordd.chars())
                    .filter(|(a, b)| a != b)
                    .count()
                    <= 2
                {
                    ret.push(wordq.clone());
                    break;
                }
            }
        }

        ret
    }
}
```
