# 2337. Move Pieces to Obtain a String
You are given two strings `start` and `target`, both of length `n`. Each string consists **only** of the characters `'L'`, `'R'`, and `'_'` where:

* The characters `'L'` and `'R'` represent pieces, where a piece `'L'` can move to the left only if there is a **blank** space directly to its left, and a piece `'R'` can move to the **right** only if there is a **blank** space directly to its right.
* The character `'_'` represents a blank space that can be occupied by **any** of the `'L'` or `'R'` pieces.

Return `true` *if it is possible to obtain the string* `target` *by moving the pieces of the string* `start` ***any** number of times*. Otherwise, return `false`.

#### Example 1:
<pre>
<strong>Input:</strong> start = "_L__R__R_", target = "L______RR"
<strong>Output:</strong> true
<strong>Explanation:</strong> We can obtain the string target from start by doing the following moves:
- Move the first piece one step to the left, start becomes equal to "L___R__R_".
- Move the last piece one step to the right, start becomes equal to "L___R___R".
- Move the second piece three steps to the right, start becomes equal to "L______RR".
Since it is possible to get the string target from start, we return true.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> start = "R_L_", target = "__LR"
<strong>Output:</strong> false
<strong>Explanation:</strong> The 'R' piece in the string start can move one step to the right to obtain "_RL_".
After that, no pieces can move anymore, so it is impossible to obtain the string target from start.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> start = "_R", target = "R_"
<strong>Output:</strong> false
<strong>Explanation:</strong> The piece in the string start can move only to the right, so it is impossible to obtain the string target from start.
</pre>

#### Constraints:
* `n == start.length == target.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* `start` and `target` consist of the characters `'L'`, `'R'`, and `'_'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let mut start_pieces = start
            .chars()
            .enumerate()
            .filter(|(_, c)| *c != '_')
            .collect::<Vec<_>>();
        let mut target_pieces = target
            .chars()
            .enumerate()
            .filter(|(_, c)| *c != '_')
            .collect::<Vec<_>>();

        if start_pieces.len() != target_pieces.len() {
            return false;
        }

        for i in 0..start_pieces.len() {
            match (start_pieces[i], target_pieces[i]) {
                ((j, 'L'), (k, 'L')) if j >= k => (),
                ((j, 'R'), (k, 'R')) if j <= k => (),
                _ => return false,
            }
        }

        true
    }
}
```
