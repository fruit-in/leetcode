# 1639. Number of Ways to Form a Target String Given a Dictionary
You are given a list of strings of the **same length** `words` and a string `target`.

Your task is to form `target` using the given `words` under the following rules:
* `target` should be formed from left to right.
* To form the <code>i<sup>th</sup></code> character (**0-indexed**) of `target`, you can choose the <code>k<sup>th</sup></code> character of the <code>j<sup>th</sup></code> string in `words` if `target[i] = words[j][k]`.
* Once you use the <code>k<sup>th</sup></code> character of the <code>j<sup>th</sup></code> string of `words`, you **can no longer** use the <code>x<sup>th</sup></code> character of any string in `words` where `x <= k`. In other words, all characters to the left of or at index `k` become unusuable for every string.
* Repeat the process until you form the string `target`.

**Notice** that you can use **multiple characters** from the **same string** in `words` provided the conditions above are met.

Return *the number of ways to form `target` from `words`*. Since the answer may be too large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> words = ["acca","bbbb","caca"], target = "aba"
<strong>Output:</strong> 6
<strong>Explanation:</strong> There are 6 ways to form target.
"aba" -> index 0 ("acca"), index 1 ("bbbb"), index 3 ("caca")
"aba" -> index 0 ("acca"), index 2 ("bbbb"), index 3 ("caca")
"aba" -> index 0 ("acca"), index 1 ("bbbb"), index 3 ("acca")
"aba" -> index 0 ("acca"), index 2 ("bbbb"), index 3 ("acca")
"aba" -> index 1 ("caca"), index 2 ("bbbb"), index 3 ("acca")
"aba" -> index 1 ("caca"), index 2 ("bbbb"), index 3 ("caca")
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> words = ["abba","baab"], target = "bab"
<strong>Output:</strong> 4
<strong>Explanation:</strong> There are 4 ways to form target.
"bab" -> index 0 ("baab"), index 1 ("baab"), index 2 ("abba")
"bab" -> index 0 ("baab"), index 1 ("baab"), index 3 ("baab")
"bab" -> index 0 ("baab"), index 2 ("baab"), index 3 ("baab")
"bab" -> index 1 ("abba"), index 2 ("baab"), index 3 ("baab")
</pre>

#### Constraints:
* `1 <= words.length <= 1000`
* `1 <= words[i].length <= 1000`
* All strings in `words` have the same length.
* `1 <= target.length <= 1000`
* `words[i]` and `target` contain only lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let target = target
            .bytes()
            .map(|b| (b - b'a') as usize)
            .collect::<Vec<_>>();
        let mut count = vec![[0; 26]; words[0].len()];
        let mut dp = vec![vec![0; count.len() + 1]; target.len() + 1];
        dp[0] = vec![1_i64; count.len() + 1];

        for word in &words {
            let word = word.as_bytes();

            for i in 0..word.len() {
                count[i][(word[i] - b'a') as usize] += 1;
            }
        }

        for i in 1..=target.len() {
            for j in 1..=count.len() {
                dp[i][j] = dp[i][j - 1];
                dp[i][j] =
                    (dp[i][j] + dp[i - 1][j - 1] * count[j - 1][target[i - 1]]) % 1_000_000_007;
            }
        }

        dp[target.len()][count.len()] as i32
    }
}
```
