# 2002. Maximum Product of the Length of Two Palindromic Subsequences
Given a string `s`, find two **disjoint palindromic subsequences** of `s` such that the **product** of their lengths is **maximized**. The two subsequences are **disjoint** if they do not both pick a character at the same index.

Return *the **maximum** possible **product** of the lengths of the two palindromic subsequences*.

A **subsequence** is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters. A string is **palindromic** if it reads the same forward and backward.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/08/24/two-palindromic-subsequences.png)
<pre>
<strong>Input:</strong> s = "leetcodecom"
<strong>Output:</strong> 9
<strong>Explanation:</strong> An optimal solution is to choose "ete" for the 1st subsequence and "cdc" for the 2nd subsequence.
The product of their lengths is: 3 * 3 = 9.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "bb"
<strong>Output:</strong> 1
<strong>Explanation:</strong> An optimal solution is to choose "b" (the first character) for the 1st subsequence and "b" (the second character) for the 2nd subsequence.
The product of their lengths is: 1 * 1 = 1.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "accbcaxxcxx"
<strong>Output:</strong> 25
<strong>Explanation:</strong> An optimal solution is to choose "accca" for the 1st subsequence and "xxcxx" for the 2nd subsequence.
The product of their lengths is: 5 * 5 = 25.
</pre>

#### Constraints:
* `2 <= s.length <= 12`
* `s` consists of lowercase English letters only.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_product(s: String) -> i32 {
        let s = s.as_bytes();
        let mut palindromic_subs = vec![];
        let mut ret = 0;

        for x in 1_i32..(1 << s.len()) {
            let mut sub = vec![];

            for i in 0..s.len() {
                if x & (1 << i) != 0 {
                    sub.push(s[i]);
                }
            }

            for i in 0..=sub.len() / 2 {
                if sub[i] != sub[sub.len() - 1 - i] {
                    break;
                }
                if i == sub.len() / 2 {
                    for y in &palindromic_subs {
                        if x & y == 0 {
                            ret = ret.max(x.count_ones() * y.count_ones());
                        }
                    }
                    palindromic_subs.push(x);
                }
            }
        }

        ret as i32
    }
}
```
