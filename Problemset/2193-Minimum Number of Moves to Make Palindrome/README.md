# 2193. Minimum Number of Moves to Make Palindrome
You are given a string `s` consisting only of lowercase English letters.

In one **move**, you can select any two **adjacent** characters of `s` and swap them.

Return *the **minimum number of moves** needed to make* `s` *a palindrome*.

**Note** that the input will be generated such that `s` can always be converted to a palindrome.

#### Example 1:
<pre>
<strong>Input:</strong> s = "aabb"
<strong>Output:</strong> 2
<strong>Explanation:</strong>
We can obtain two palindromes from s, "abba" and "baab".
- We can obtain "abba" from s in 2 moves: "aabb" -> "abab" -> "abba".
- We can obtain "baab" from s in 2 moves: "aabb" -> "abab" -> "baab".
Thus, the minimum number of moves needed to make s a palindrome is 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "letelt"
<strong>Output:</strong> 2
<strong>Explanation:</strong>
One of the palindromes we can obtain from s in 2 moves is "lettel".
One of the ways we can obtain it is "letelt" -> "letetl" -> "lettel".
Other palindromes such as "tleelt" can also be obtained in 2 moves.
It can be shown that it is not possible to obtain a palindrome in less than 2 moves.
</pre>

#### Constraints:
* `1 <= s.length <= 2000`
* `s` consists only of lowercase English letters.
* `s` can be converted to a palindrome using a finite number of moves.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_moves_to_make_palindrome(s: String) -> i32 {
        let mut s = s.into_bytes();
        let n = s.len();
        let mut l = 0;
        let mut r = n - 1;
        let mut ret = 0;

        while l < r {
            for i in (l..=r).rev() {
                if i == l {
                    ret += (n / 2 - i) as i32;
                } else if s[i] == s[l] {
                    for j in i..r {
                        s.swap(j, j + 1);
                        ret += 1;
                    }

                    r -= 1;
                    break;
                }
            }

            l += 1;
        }

        ret
    }
}
```
