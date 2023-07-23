# 2423. Remove Letter To Equalize Frequency
You are given a **0-indexed** string `word`, consisting of lowercase English letters. You need to select **one** index and **remove** the letter at that index from `word` so that the **frequency** of every letter present in `word` is equal.

Return `true` *if it is possible to remove one letter so that the frequency of all letters in* `word` *are equal, and* `false` *otherwise*.

**Note:**

* The **frequency** of a letter `x` is the number of times it occurs in the string.
* You **must** remove exactly one letter and cannot chose to do nothing.

#### Example 1:
<pre>
<strong>Input:</strong> word = "abcc"
<strong>Output:</strong> true
<strong>Explanation:</strong> Select index 3 and delete it: word becomes "abc" and each character has a frequency of 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> word = "aazz"
<strong>Output:</strong> false
<strong>Explanation:</strong> We must delete a character, so either the frequency of "a" is 1 and the frequency of "z" is 2, or vice versa. It is impossible to make all present letters have equal frequency.
</pre>

#### Constraints:
* `2 <= word.length <= 100`
* `word` consists of lowercase English letters only.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let word = word.as_bytes();

        for i in 0..word.len() {
            let mut count = [0; 26];

            for j in (0..i).chain(i + 1..word.len()) {
                count[(word[j] - b'a') as usize] += 1;
            }

            let x = *count.iter().find(|&&x| x > 0).unwrap();

            if count.iter().all(|&c| c == 0 || c == x) {
                return true;
            }
        }

        false
    }
}
```
