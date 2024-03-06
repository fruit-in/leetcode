# 552. Student Attendance Record II
An attendance record for a student can be represented as a string where each character signifies whether the student was absent, late, or present on that day. The record only contains the following three characters:

* `'A'`: Absent.
* `'L'`: Late.
* `'P'`: Present.

Any student is eligible for an attendance award if they meet **both** of the following criteria:

* The student was absent (`'A'`) for **strictly** fewer than 2 days **total**.
* The student was **never** late (`'L'`) for 3 or more **consecutive** days.

Given an integer `n`, return *the **number** of possible attendance records of length* `n` *that make a student eligible for an attendance award. The answer may be very large, so return it **modulo*** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> 8
<strong>Explanation:</strong> There are 8 records with length 2 that are eligible for an award:
"PP", "AP", "PA", "LP", "PL", "AL", "LA", "LL"
Only "AA" is not eligible because there are 2 absences (there need to be fewer than 2).
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 3
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 10101
<strong>Output:</strong> 183236316
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![[[0_i64; 3]; 2]; n + 1];
        let mut ret = 0;
        dp[0][0][0] = 1;

        for i in 0..n {
            dp[i + 1][1][0] = dp[i][0][0] + dp[i][0][1] + dp[i][0][2];
            for j in 0..2 {
                dp[i + 1][j][0] += dp[i][j][0] + dp[i][j][1] + dp[i][j][2];
                dp[i + 1][j][0] %= 1_000_000_007;
                for k in 0..2 {
                    dp[i + 1][j][k + 1] += dp[i][j][k];
                    dp[i + 1][j][k + 1] %= 1_000_000_007;
                }
            }
        }

        for j in 0..2 {
            for k in 0..3 {
                ret = (ret + dp[n][j][k] as i32) % 1_000_000_007;
            }
        }

        ret
    }
}
```
