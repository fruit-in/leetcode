# 2318. Number of Distinct Roll Sequences
You are given an integer `n`. You roll a fair 6-sided dice `n` times. Determine the total number of **distinct** sequences of rolls possible such that the following conditions are satisfied:

1. The **greatest common divisor** of any **adjacent** values in the sequence is equal to `1`.
2. There is **at least** a gap of `2` rolls between **equal** valued rolls. More formally, if the value of the <code>i<sup>th</sup></code> roll is **equal** to the value of the <code>j<sup>th</sup></code> roll, then `abs(i - j) > 2`.

Return *the **total number** of distinct sequences possible*. Since the answer may be very large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

Two sequences are considered distinct if at least one element is different.

#### Example 1:
<pre>
<strong>Input:</strong> n = 4
<strong>Output:</strong> 184
<strong>Explanation:</strong> Some of the possible sequences are (1, 2, 3, 4), (6, 1, 2, 3), (1, 2, 3, 1), etc.
Some invalid sequences are (1, 2, 1, 3), (1, 2, 3, 6).
(1, 2, 1, 3) is invalid since the first and third roll have an equal value and abs(1 - 3) = 2 (i and j are 1-indexed).
(1, 2, 3, 6) is invalid since the greatest common divisor of 3 and 6 = 3.
There are a total of 184 distinct sequences possible, so we return 184.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> 22
<strong>Explanation:</strong> Some of the possible sequences are (1, 2), (2, 1), (3, 2).
Some invalid sequences are (3, 6), (2, 4) since the greatest common divisor is not equal to 1.
There are a total of 22 distinct sequences possible, so we return 22.
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    const BANS: [[usize; 4]; 6] = [
        [0, 0, 0, 0],
        [1, 3, 5, 5],
        [2, 5, 5, 5],
        [1, 3, 5, 5],
        [4, 4, 4, 4],
        [1, 2, 3, 5],
    ];
    const MOD: i32 = 1_000_000_007;

    pub fn distinct_sequences(n: i32) -> i32 {
        if n == 1 {
            return 6;
        }

        let mut dp = [[0; 6]; 6];
        let mut ret = 0;

        for i in 0..6 {
            for j in 0..6 {
                dp[i][j] = !Self::BANS[j].contains(&i) as i32;
            }
        }

        for _ in 2..n {
            let mut tmp = [[0; 6]; 6];

            for i in 0..6 {
                for j in 0..6 {
                    for k in (0..6).filter(|&x| x != i && !Self::BANS[x].contains(&j)) {
                        tmp[j][k] = (tmp[j][k] + dp[i][j]) % Self::MOD;
                    }
                }
            }

            dp = tmp;
        }

        for i in 0..6 {
            for j in 0..6 {
                ret = (ret + dp[i][j]) % Self::MOD;
            }
        }

        ret
    }
}
```
