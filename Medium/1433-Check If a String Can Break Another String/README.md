# 1433. Check If a String Can Break Another String
Given two strings: `s1` and `s2` with the same size, check if some permutation of string `s1` can break some permutation of string `s2` or vice-versa (in other words `s2` can break `s1`).

A string `x` can break string `y` (both of size `n`) if `x[i] >= y[i]` (in alphabetical order) for all `i` between `0` and `n-1`.

#### Example 1:
<pre>
<strong>Input:</strong> s1 = "abc", s2 = "xya"
<strong>Output:</strong> true
<strong>Explanation:</strong> "ayx" is a permutation of s2="xya" which can break to string "abc" which is a permutation of s1="abc".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s1 = "abe", s2 = "acd"
<strong>Output:</strong> false
<strong>Explanation:</strong> All permutations for s1="abe" are: "abe", "aeb", "bae", "bea", "eab" and "eba" and all permutation for s2="acd" are: "acd", "adc", "cad", "cda", "dac" and "dca". However, there is not any permutation from s1 which can break some permutation from s2 and vice-versa.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s1 = "leetcodee", s2 = "interview"
<strong>Output:</strong> true
</pre>

#### Constraints:
* `s1.length == n`
* `s2.length == n`
* `1 <= n <= 10^5`
* All strings consist of lowercase English letters.

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn check_if_can_break(s1: String, s2: String) -> bool {
        let mut s1 = s1.into_bytes();
        let mut s2 = s2.into_bytes();
        s1.sort_unstable();
        s2.sort_unstable();
        let s1s2 = s1.iter().zip(s2.iter()).collect::<Vec<_>>();

        s1s2.iter().all(|tup| tup.0 >= tup.1) || s1s2.iter().all(|tup| tup.0 <= tup.1)
    }
}
```
