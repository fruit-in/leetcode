# 1563. Stone Game V
There are several stones **arranged in a row**, and each stone has an associated value which is an integer given in the array `stoneValue`.

In each round of the game, Alice divides the row into **two non-empty rows** (i.e. left row and right row), then Bob calculates the value of each row which is the sum of the values of all the stones in this row. Bob throws away the row which has the maximum value, and Alice's score increases by the value of the remaining row. If the value of the two rows are equal, Bob lets Alice decide which row will be thrown away. The next round starts with the remaining row.

The game ends when there is only **one stone remaining**. Alice's is initially **zero**.

Return *the maximum score that Alice can obtain*.

#### Example 1:
<pre>
<strong>Input:</strong> stoneValue = [6,2,3,4,5,5]
<strong>Output:</strong> 18
<strong>Explanation:</strong> In the first round, Alice divides the row to [6,2,3], [4,5,5]. The left row has the value 11 and the right row has value 14. Bob throws away the right row and Alice's score is now 11.
In the second round Alice divides the row to [6], [2,3]. This time Bob throws away the left row and Alice's score becomes 16 (11 + 5).
The last round Alice has only one choice to divide the row which is [2], [3]. Bob throws away the right row and Alice's score is now 18 (16 + 2). The game ends because only one stone is remaining in the row.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> stoneValue = [7,7,7,7,7,7,7]
<strong>Output:</strong> 28
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> stoneValue = [4]
<strong>Output:</strong> 0
</pre>

#### Constraints:
* `1 <= stoneValue.length <= 500`
* <code>1 <= stoneValue[i] <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        let n = stone_value.len();
        let mut prefix_sum = vec![0; n + 1];
        let mut dp = vec![vec![0; n]; n];

        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + stone_value[i];
        }

        for i in 2..=n {
            for j in 0..n - i + 1 {
                let mut left_sum = 0;
                let mut right_sum = prefix_sum[j + i] - prefix_sum[j];

                for k in j..j + i - 1 {
                    left_sum += stone_value[k];
                    right_sum -= stone_value[k];

                    if left_sum <= right_sum {
                        dp[j][j + i - 1] = dp[j][j + i - 1].max(left_sum + dp[j][k]);
                    }
                    if left_sum >= right_sum {
                        dp[j][j + i - 1] = dp[j][j + i - 1].max(right_sum + dp[k + 1][j + i - 1]);
                    }
                }
            }
        }

        dp[0][n - 1]
    }
}
```
