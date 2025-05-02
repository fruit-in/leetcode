# 1690. Stone Game VII
Alice and Bob take turns playing a game, with **Alice starting first**.

There are `n` stones arranged in a row. On each player's turn, they can **remove** either the leftmost stone or the rightmost stone from the row and receive points equal to the **sum** of the remaining stones' values in the row. The winner is the one with the higher score when there are no stones left to remove.

Bob found that he will always lose this game (poor Bob, he always loses), so he decided to **minimize the score's difference**. Alice's goal is to **maximize the difference** in the score.

Given an array of integers `stones` where `stones[i]` represents the value of the <code>i<sup>th</sup></code> stone **from the left**, return *the **difference** in Alice and Bob's score if they both play **optimally***.

#### Example 1:
<pre>
<strong>Input:</strong> stones = [5,3,1,4,2]
<strong>Output:</strong> 6
<strong>Explanation:</strong>
- Alice removes 2 and gets 5 + 3 + 1 + 4 = 13 points. Alice = 13, Bob = 0, stones = [5,3,1,4].
- Bob removes 5 and gets 3 + 1 + 4 = 8 points. Alice = 13, Bob = 8, stones = [3,1,4].
- Alice removes 3 and gets 1 + 4 = 5 points. Alice = 18, Bob = 8, stones = [1,4].
- Bob removes 1 and gets 4 points. Alice = 18, Bob = 12, stones = [4].
- Alice removes 4 and gets 0 points. Alice = 18, Bob = 12, stones = [].
The score difference is 18 - 12 = 6.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> stones = [7,90,5,1,100,10,10,2]
<strong>Output:</strong> 122
</pre>

#### Constraints:
* `n == stones.length`
* `2 <= n <= 1000`
* `1 <= stones[i] <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut prefix_sum = vec![0; n + 1];
        let mut dp = vec![vec![0; n]; n];

        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + stones[i];
        }

        for i in 1..n {
            for j in 0..n - i {
                dp[j][j + i] = (prefix_sum[j + i + 1] - prefix_sum[j + 1] - dp[j + 1][j + i])
                    .max(prefix_sum[j + i] - prefix_sum[j] - dp[j][j + i - 1]);
            }
        }

        dp[0][n - 1]
    }
}
```
