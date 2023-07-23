# 2381. Shifting Letters II
You are given a string `s` of lowercase English letters and a 2D integer array `shifts` where <code>shifts[i] = [start<sub>i</sub>, end<sub>i</sub>, direction<sub>i</sub>]</code>. For every `i`, **shift** the characters in `s` from the index <code>start<sub>i</sub></code> to the index <code>end<sub>i</sub></code> (**inclusive**) forward if <code>direction<sub>i</sub> = 1</code>, or shift the characters backward if <code>direction<sub>i</sub> = 0</code>.

Shifting a character **forward** means replacing it with the **next** letter in the alphabet (wrapping around so that `'z'` becomes `'a'`). Similarly, shifting a character **backward** means replacing it with the **previous** letter in the alphabet (wrapping around so that `'a'` becomes `'z'`).

Return *the final string after all such shifts to* `s` *are applied*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "abc", shifts = [[0,1,0],[1,2,1],[0,2,1]]
<strong>Output:</strong> "ace"
<strong>Explanation:</strong> Firstly, shift the characters from index 0 to index 1 backward. Now s = "zac".
Secondly, shift the characters from index 1 to index 2 forward. Now s = "zbd".
Finally, shift the characters from index 0 to index 2 forward. Now s = "ace".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "dztz", shifts = [[0,0,0],[1,1,1]]
<strong>Output:</strong> "catz"
<strong>Explanation:</strong> Firstly, shift the characters from index 0 to index 0 backward. Now s = "cztz".
Finally, shift the characters from index 1 to index 1 forward. Now s = "catz".
</pre>

#### Constraints:
* <code>1 <= s.length, shifts.length <= 5 * 10<sup>4</sup></code>
* `shifts[i].length == 3`
* <code>0 <= start<sub>i</sub> <= end<sub>i</sub> < s.length</code>
* <code>0 <= direction<sub>i</sub> <= 1</code>
* `s` consists of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let mut s = s.into_bytes();
        let mut prefix_sum = vec![0; s.len()];

        for i in 0..shifts.len() {
            if shifts[i][0] > 0 {
                prefix_sum[shifts[i][0] as usize - 1] -= 2 * shifts[i][2] - 1;
            }
            prefix_sum[shifts[i][1] as usize] += 2 * shifts[i][2] - 1;
        }

        for i in (0..prefix_sum.len()).rev() {
            prefix_sum[i] += *prefix_sum.get(i + 1).unwrap_or(&0);
            s[i] = ((s[i] - b'a') as i32 + prefix_sum[i]).rem_euclid(26) as u8 + b'a';
        }

        String::from_utf8(s).unwrap()
    }
}
```
