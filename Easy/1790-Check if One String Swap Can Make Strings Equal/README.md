# 1790. Check if One String Swap Can Make Strings Equal
You are given two strings `s1` and `s2` of equal length. A **string swap** is an operation where you choose two indices in a string (not necessarily different) and swap the characters at these indices.

Return `true` *if it is possible to make both strings equal by performing **at most one string swap** on **exactly one** of the strings*. Otherwise, return `false`.

#### Example 1:
<pre>
<strong>Input:</strong> s1 = "bank", s2 = "kanb"
<strong>Output:</strong> true
<strong>Explanation:</strong> For example, swap the first character with the last character of s2 to make "bank".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s1 = "attack", s2 = "defend"
<strong>Output:</strong> false
<strong>Explanation:</strong> It is impossible to make them equal with one string swap.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s1 = "kelb", s2 = "kelb"
<strong>Output:</strong> true
<strong>Explanation:</strong> The two strings are already equal, so no string swap operation is required.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> s1 = "abcd", s2 = "dcba"
<strong>Output:</strong> false
</pre>

#### Constraints:
* `1 <= s1.length, s2.length <= 100`
* `s1.length == s2.length`
* `s1` and `s2` consist of only lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let diff = s1
            .chars()
            .zip(s2.chars())
            .filter(|(c0, c1)| c0 != c1)
            .take(3)
            .collect::<Vec<_>>();

        diff.is_empty() || (diff.len() == 2 && diff[0].0 == diff[1].1 && diff[0].1 == diff[1].0)
    }
}
```
