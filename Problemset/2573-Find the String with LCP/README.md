# 2573. Find the String with LCP
We define the `lcp` matrix of any **0-indexed** string `word` of `n` lowercase English letters as an `n x n` grid such that:
* `lcp[i][j]` is equal to the length of the **longest common prefix** between the substrings `word[i,n-1]` and `word[j,n-1]`.

Given an `n x n` matrix `lcp`, return the alphabetically smallest string `word` that corresponds to `lcp`. If there is no such string, return an empty string.

A string `a` is lexicographically smaller than a string `b` (of the same length) if in the first position where `a` and `b` differ, string `a` has a letter that appears earlier in the alphabet than the corresponding letter in `b`. For example, `"aabd"` is lexicographically smaller than `"aaca"` because the first position they differ is at the third letter, and `'b'` comes before `'c'`.

#### Example 1:
<pre>
<strong>Input:</strong> lcp = [[4,0,2,0],[0,3,0,1],[2,0,2,0],[0,1,0,1]]
<strong>Output:</strong> "abab"
<strong>Explanation:</strong> lcp corresponds to any 4 letter string with two alternating letters. The lexicographically smallest of them is "abab".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> lcp = [[4,3,2,1],[3,3,2,1],[2,2,2,1],[1,1,1,1]]
<strong>Output:</strong> "aaaa"
<strong>Explanation:</strong> lcp corresponds to any 4 letter string with a single distinct letter. The lexicographically smallest of them is "aaaa".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> lcp = [[4,3,2,1],[3,3,2,1],[2,2,2,1],[1,1,1,3]]
<strong>Output:</strong> ""
<strong>Explanation:</strong> lcp[3][3] cannot be equal to 3 since word[3,...,3] consists of only a single letter; Thus, no answer exists.
</pre>

#### Constraints:
* `1 <= n == lcp.length == lcp[i].length <= 1000`
* `0 <= lcp[i][j] <= n`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
        let n = lcp.len();
        let mut masks = vec![0_i32; n];
        let mut word = vec![b'a'; n];

        for i in 0..n {
            if lcp[i][i] as usize != n - i || masks[i].trailing_ones() > 25 {
                return "".to_string();
            }

            word[i] += masks[i].trailing_ones() as u8;

            for j in i + 1..n {
                if lcp[i][j] > 0 && i + 1 < n && j + 1 < n && lcp[i + 1][j + 1] != lcp[i][j] - 1 {
                    return "".to_string();
                } else if lcp[i][j] != lcp[j][i] {
                    return "".to_string();
                } else if lcp[i][j] as usize > n - i.max(j) {
                    return "".to_string();
                }

                masks[j] |= match lcp[i][j] {
                    0 => 1 << (word[i] - b'a'),
                    _ => i32::MAX ^ (1 << (word[i] - b'a')),
                };
            }
        }

        String::from_utf8(word).unwrap()
    }
}
```
