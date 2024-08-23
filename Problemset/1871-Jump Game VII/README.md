# 1871. Jump Game VII
You are given a **0-indexed** binary string `s` and two integers `minJump` and `maxJump`. In the beginning, you are standing at index `0`, which is equal to `'0'`. You can move from index `i` to index `j` if the following conditions are fulfilled:

* `i + minJump <= j <= min(i + maxJump, s.length - 1)`, and
* `s[j] == '0'`.

Return `true` *if you can reach index* `s.length - 1` *in* `s`*, or* `false` *otherwise*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "011010", minJump = 2, maxJump = 3
<strong>Output:</strong> true
<strong>Explanation:</strong>
In the first step, move from index 0 to index 3.
In the second step, move from index 3 to index 5.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "01101110", minJump = 2, maxJump = 3
<strong>Output:</strong> false
</pre>

#### Constraints:
* <code>2 <= s.length <= 10<sup>5</sup></code>
* `s[i]` is either `'0'` or `'1'`.
* `s[0] == '0'`
* `1 <= minJump <= maxJump < s.length`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let mut indices = vec![0];

        for (i, _) in s.chars().enumerate().skip(1).filter(|&(_, c)| c == '0') {
            match indices.binary_search(&(i as i32 - max_jump)) {
                Err(j) if j == indices.len() || indices[j] > i as i32 - min_jump => (),
                _ => indices.push(i as i32),
            }
        }

        *indices.last().unwrap() == s.len() as i32 - 1
    }
}
```
