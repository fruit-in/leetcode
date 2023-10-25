# 395. Longest Substring with At Least K Repeating Characters
Given a string `s` and an integer `k`, return *the length of the longest substring of* `s` *such that the frequency of each character in this substring is greater than or equal to* `k`.

if no such substring exists, return 0.

#### Example 1:
<pre>
<strong>Input:</strong> s = "aaabb", k = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> The longest substring is "aaa", as 'a' is repeated 3 times.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "ababbc", k = 2
<strong>Output:</strong> 5
<strong>Explanation:</strong> The longest substring is "ababb", as 'a' is repeated 2 times and 'b' is repeated 3 times.
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>4</sup></code>
* `s` consists of only lowercase English letters.
* <code>1 <= k <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        Self::dfs(s.as_bytes(), 0, s.len() - 1, k as usize)
    }

    fn dfs(s: &[u8], l: usize, r: usize, k: usize) -> i32 {
        if l > r || r - l + 1 < k {
            return 0;
        }

        let mut indices = vec![vec![]; 26];
        let mut ret = (r - l + 1) as i32;

        for i in l..=r {
            indices[(s[i] - b'a') as usize].push(i);
        }

        for i in 0..26 {
            if !indices[i].is_empty() && indices[i].len() < k {
                ret = 0;
                ret = ret.max(Self::dfs(s, l, indices[i][0].saturating_sub(1), k));
                for j in 1..indices[i].len() {
                    ret = ret.max(Self::dfs(s, indices[i][j - 1] + 1, indices[i][j] - 1, k));
                }
                ret = ret.max(Self::dfs(s, *indices[i].last().unwrap() + 1, r, k));
                break;
            }
        }

        ret
    }
}
```
