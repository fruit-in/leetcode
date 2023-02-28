# 2186. Minimum Number of Steps to Make Two Strings Anagram II
You are given two strings `s` and `t`. In one step, you can append **any character** to either `s` or `t`.

Return *the minimum number of steps to make* `s` *and* `t` ***anagrams** of each other*.

An **anagram** of a string is a string that contains the same characters with a different (or the same) ordering.

#### Example 1:
<pre>
<strong>Input:</strong> s = "leetcode", t = "coats"
<strong>Output:</strong> 7
<strong>Explanation:</strong>
- In 2 steps, we can append the letters in "as" onto s = "leetcode", forming s = "leetcodeas".
- In 5 steps, we can append the letters in "leede" onto t = "coats", forming t = "coatsleede".
"leetcodeas" and "coatsleede" are now anagrams of each other.
We used a total of 2 + 5 = 7 steps.
It can be shown that there is no way to make them anagrams of each other with less than 7 steps.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "night", t = "thing"
<strong>Output:</strong> 0
<strong>Explanation:</strong> The given strings are already anagrams of each other. Thus, we do not need any further steps.
</pre>

#### Constraints:
* <code>1 <= s.length, t.length <= 2 * 10<sup>5</sup></code>
* `s` and `t` consist of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut count = [(0_i32, 0_i32); 26];

        s.bytes().for_each(|b| count[(b - b'a') as usize].0 += 1);
        t.bytes().for_each(|b| count[(b - b'a') as usize].1 += 1);

        count.into_iter().map(|(x, y)| (x - y).abs()).sum()
    }
}
```
