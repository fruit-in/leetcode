# 1525. Number of Good Ways to Split a String
You are given a string `s`, a split is called *good* if you can split `s` into 2 non-empty strings `p` and `q` where its concatenation is equal to `s` and the number of distinct letters in `p` and `q` are the same.

Return the number of *good* splits you can make in `s`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "aacaba"
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are 5 ways to split "aacaba" and 2 of them are good. 
("a", "acaba") Left string and right string contains 1 and 3 different letters respectively.
("aa", "caba") Left string and right string contains 1 and 3 different letters respectively.
("aac", "aba") Left string and right string contains 2 and 2 different letters respectively (good split).
("aaca", "ba") Left string and right string contains 2 and 2 different letters respectively (good split).
("aacab", "a") Left string and right string contains 3 and 1 different letters respectively.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "abcd"
<strong>Output:</strong> 1
<strong>Explanation:</strong> Split the string as follows ("ab", "cd").
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "aaaaa"
<strong>Output:</strong> 4
<strong>Explanation:</strong> All possible splits are good.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> s = "acbadbaada"
<strong>Output:</strong> 2
</pre>

#### Constraints:
* `s` contains only lowercase English letters.
* `1 <= s.length <= 10^5`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn num_splits(s: String) -> i32 {
        let mut p_count = vec![];
        let mut q_count = vec![];
        let mut count = 0i32;

        for ch in s.bytes().take(s.len() - 1) {
            count |= 1 << (ch - b'a');
            p_count.push(count.count_ones());
        }
        count = 0;
        for ch in s.bytes().rev().take(s.len() - 1) {
            count |= 1 << (ch - b'a');
            q_count.push(count.count_ones());
        }

        p_count
            .iter()
            .zip(q_count.iter().rev())
            .filter(|(x, y)| x == y)
            .count() as i32
    }
}
```
