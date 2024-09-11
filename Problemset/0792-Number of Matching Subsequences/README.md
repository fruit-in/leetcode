# 792. Number of Matching Subsequences
Given a string `s` and an array of strings `words`, return *the number of* `words[i]` *that is a subsequence of* `s`.

A **subsequence** of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.

* For example, `"ace"` is a subsequence of `"abcde"`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "abcde", words = ["a","bb","acd","ace"]
<strong>Output:</strong> 3
<strong>Explanation:</strong> There are three strings in words that are a subsequence of s: "a", "acd", "ace".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "dsahjpjauf", words = ["ahjpjau","ja","ahbwzgqnuk","tnmlanowax"]
<strong>Output:</strong> 2
</pre>

#### Constraints:
* <code>1 <= s.length <= 5 * 10<sup>4</sup></code>
* `1 <= words.length <= 5000`
* `1 <= words[i].length <= 50`
* `s` and `words[i]` consist of only lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut indices = vec![vec![]; 26];
        let mut ret = 0;

        for (i, c) in s.bytes().enumerate() {
            indices[(c - b'a') as usize].push(i);
        }

        for word in &words {
            let mut i = 0;
            let mut flag = true;

            for c in word.bytes() {
                match indices[(c - b'a') as usize].binary_search(&i) {
                    Err(j) if j == indices[(c - b'a') as usize].len() => {
                        flag = false;
                        break;
                    }
                    Ok(j) | Err(j) => i = indices[(c - b'a') as usize][j] + 1,
                }
            }

            ret += flag as i32;
        }

        ret
    }
}
```
