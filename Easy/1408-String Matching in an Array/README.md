# 1408. String Matching in an Array
Given an array of string `words`. Return all strings in `words` which is substring of another word in **any** order.

String `words[i]` is substring of `words[j]`, if can be obtained removing some characters to left and/or right side of `words[j]`.

#### Example 1:
<pre>
<strong>Input:</strong> words = ["mass","as","hero","superhero"]
<strong>Output:</strong> ["as","hero"]
<strong>Explanation:</strong> "as" is substring of "mass" and "hero" is substring of "superhero".
["hero","as"] is also a valid answer.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> words = ["leetcode","et","code"]
<strong>Output:</strong> ["et","code"]
<strong>Explanation:</strong> "et", "code" are substring of "leetcode".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> words = ["blue","green","bu"]
<strong>Output:</strong> []
</pre>

#### Constraints:
* `1 <= words.length <= 100`
* `1 <= words[i].length <= 30`
* `words[i]` contains only lowercase English letters.
* It's **guaranteed** that `words[i]` will be unique.

## Solutions (Rust)

### 1. Brute Force
```Rust
impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut ret = Vec::new();

        for i in 0..words.len() {
            for j in 0..words.len() {
                if j != i && words[j].contains(&words[i]) {
                    ret.push(words[i].clone());
                    break;
                }
            }
        }

        ret
    }
}
```
