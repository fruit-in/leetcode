# 2027. Minimum Moves to Convert String
You are given a string `s` consisting of `n` characters which are either `'X'` or `'O'`.

A **move** is defined as selecting **three consecutive characters** of `s` and converting them to `'O'`. Note that if a move is applied to the character `'O'`, it will stay the **same**.

Return *the **minimum** number of moves required so that all the characters of* `s` *are converted to* `'O'`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "XXX"
<strong>Output:</strong> 1
<strong>Explanation:</strong> XXX -> OOO
We select all the 3 characters and convert them in one move.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "XXOX"
<strong>Output:</strong> 2
<strong>Explanation:</strong> XXOX -> OOOX -> OOOO
We select the first 3 characters in the first move, and convert them to 'O'.
Then we select the last 3 characters and convert them so that the final string contains all 'O's.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "OOOO"
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no 'X's in s to convert.
</pre>

#### Constraints:
* `3 <= s.length <= 1000`
* `s[i]` is either `'X'` or `'O'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let s = s.as_bytes();
        let mut i = 0;
        let mut ret = 0;

        while i < s.len() {
            if s[i] == b'X' {
                i += 2;
                ret += 1;
            }
            i += 1;
        }

        ret
    }
}
```
