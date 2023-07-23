# 1897. Redistribute Characters to Make All Strings Equal
You are given an array of strings `words` (**0-indexed**).

In one operation, pick two **distinct** indices `i` and `j`, where `words[i]` is a non-empty string, and move **any** character from `words[i]` to **any** position in `words[j]`.

Return `true` *if you can make **every** string in* `words` ***equal** using **any** number of operations, and* `false` *otherwise*.

#### Example 1:
<pre>
<strong>Input:</strong> words = ["abc","aabc","bc"]
<strong>Output:</strong> true
<strong>Explanation:</strong> Move the first 'a' in words[1] to the front of words[2],
to make words[1] = "abc" and words[2] = "abc".
All the strings are now equal to "abc", so return true.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> words = ["ab","a"]
<strong>Output:</strong> false
<strong>Explanation:</strong> It is impossible to make all the strings equal using the operation.
</pre>

#### Constraints:
* `1 <= words.length <= 100`
* `1 <= words[i].length <= 100`
* `words[i]` consists of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut count = [0; 26];

        for word in &words {
            for c in word.bytes() {
                count[(c - b'a') as usize] += 1;
            }
        }

        count.iter().all(|&x| x % words.len() == 0)
    }
}
```
