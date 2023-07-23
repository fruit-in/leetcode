# 2410. Maximum Matching of Players With Trainers
You are given a **0-indexed** integer array `players`, where `players[i]` represents the **ability** of the <code>i<sup>th</sup></code> player. You are also given a **0-indexed** integer array `trainers`, where `trainers[j]` represents the **training capacity** of the <code>j<sup>th</sup></code> trainer.

The <code>i<sup>th</sup></code> player can **match** with the <code>j<sup>th</sup></code> trainer if the player's ability is **less than or equal to** the trainer's training capacity. Additionally, the <code>i<sup>th</sup></code> player can be matched with at most one trainer, and the <code>j<sup>th</sup></code> trainer can be matched with at most one player.

Return *the **maximum** number of matchings between* `players` *and* `trainers` *that satisfy these conditions*.

#### Example 1:
<pre>
<strong>Input:</strong> players = [4,7,9], trainers = [8,2,5,8]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
One of the ways we can form two matchings is as follows:
- players[0] can be matched with trainers[0] since 4 <= 8.
- players[1] can be matched with trainers[3] since 7 <= 8.
It can be proven that 2 is the maximum number of matchings that can be formed.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> players = [1,1,1], trainers = [10]
<strong>Output:</strong> 1
<strong>Explanation:</strong>
The trainer can be matched with any of the 3 players.
Each player can only be matched with one trainer, so the maximum answer is 1.
</pre>

#### Constraints:
* <code>1 <= players.length, trainers.length <= 10<sup>5</sup></code>
* <code>1 <= players[i], trainers[j] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
        let mut players = players;
        let mut trainers = trainers;
        let mut ret = 0;

        players.sort_unstable();
        trainers.sort_unstable();

        while let Some(trainer) = trainers.pop() {
            while *players.last().unwrap_or(&0) > trainer {
                players.pop();
            }
            if players.pop().unwrap_or(i32::MAX) <= trainer {
                ret += 1;
            }
        }

        ret
    }
}
```
