# 828. Count Unique Characters of All Substrings of a Given String
Let's define a function `countUniqueChars(s)` that returns the number of unique characters in `s`.

* For example, calling `countUniqueChars(s)` if `s = "LEETCODE"` then `"L"`, `"T"`, `"C"`, `"O"`, `"D"` are the unique characters since they appear only once in `s`, therefore `countUniqueChars(s) = 5`.

Given a string `s`, return the sum of `countUniqueChars(t)` where `t` is a substring of `s`. The test cases are generated such that the answer fits in a 32-bit integer.

Notice that some substrings can be repeated so in this case you have to count the repeated ones too.

#### Example 1:
<pre>
<strong>Input:</strong> s = "ABC"
<strong>Output:</strong> 10
<strong>Explanation:</strong> All possible substrings are: "A","B","C","AB","BC" and "ABC".
Every substring is composed with only unique letters.
Sum of lengths of all substring is 1 + 1 + 1 + 2 + 2 + 3 = 10
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "ABA"
<strong>Output:</strong> 8
<strong>Explanation:</strong> The same as example 1, except countUniqueChars("ABA") = 1.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "LEETCODE"
<strong>Output:</strong> 92
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` consists of uppercase English letters only.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        let s = s.as_bytes();
        let mut indices = vec![vec![-1]; 26];
        let mut ret = 0;

        for i in 0..s.len() {
            indices[(s[i] - b'A') as usize].push(i as i32);
        }

        for i in 0..26 {
            indices[i].push(s.len() as i32);

            for j in 1..indices[i].len() - 1 {
                ret += (indices[i][j] - indices[i][j - 1]) * (indices[i][j + 1] - indices[i][j]);
            }
        }

        ret
    }
}
```
