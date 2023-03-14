# 2131. Longest Palindrome by Concatenating Two Letter Words
You are given an array of strings `words`. Each element of `words` consists of **two** lowercase English letters.

Create the **longest possible palindrome** by selecting some elements from `words` and concatenating them in **any order**. Each element can be selected **at most once**.

Return *the **length** of the longest palindrome that you can create*. If it is impossible to create any palindrome, return `0`.

A **palindrome** is a string that reads the same forward and backward.

#### Example 1:
<pre>
<strong>Input:</strong> words = ["lc","cl","gg"]
<strong>Output:</strong> 6
<strong>Explanation:</strong> One longest palindrome is "lc" + "gg" + "cl" = "lcggcl", of length 6.
Note that "clgglc" is another longest palindrome that can be created.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> words = ["ab","ty","yt","lc","cl","ab"]
<strong>Output:</strong> 8
<strong>Explanation:</strong> One longest palindrome is "ty" + "lc" + "cl" + "yt" = "tylcclyt", of length 8.
Note that "lcyttycl" is another longest palindrome that can be created.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> words = ["cc","ll","xx"]
<strong>Output:</strong> 2
<strong>Explanation:</strong> One longest palindrome is "cc", of length 2.
Note that "ll" is another longest palindrome that can be created, and so is "xx".
</pre>

#### Constraints:
* <code>1 <= words.length <= 10<sup>5</sup></code>
* `words[i].length == 2`
* `words[i]` consists of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut mat = [[0; 26]; 26];
        let mut flag = 0;
        let mut ret = 0;

        for word in words {
            mat[(word.as_bytes()[0] - b'a') as usize][(word.as_bytes()[1] - b'a') as usize] += 1;
        }

        for i in 0..26 {
            flag |= mat[i][i] % 2;
            ret += mat[i][i] / 2 * 4;
            for j in (i + 1)..26 {
                ret += 4 * mat[i][j].min(mat[j][i])
            }
        }

        ret + 2 * flag
    }
}
```
