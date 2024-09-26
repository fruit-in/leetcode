# 650. 2 Keys Keyboard
There is only one character `'A'` on the screen of a notepad. You can perform one of two operations on this notepad for each step:

* Copy All: You can copy all the characters present on the screen (a partial copy is not allowed).
* Paste: You can paste the characters which are copied last time.

Given an integer `n`, return *the minimum number of operations to get the character* `'A'` *exactly* `n` *times on the screen*.

#### Example 1:
<pre>
<strong>Input:</strong> n = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> Initially, we have one character 'A'.
In step 1, we use Copy All operation.
In step 2, we use Paste operation to get 'AA'.
In step 3, we use Paste operation to get 'AAA'.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 0
</pre>

#### Constraints:
* `1 <= n <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let mut dp = vec![usize::MAX; n as usize + 1];
        dp[1] = 0;

        for i in 1..dp.len() {
            for j in (i..dp.len()).step_by(i) {
                dp[j] = dp[j].min(dp[i] + j / i);
            }
        }

        dp[n as usize] as i32
    }
}
```
