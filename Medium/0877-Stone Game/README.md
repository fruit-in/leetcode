# 877. Stone Game
Alex and Lee play a game with piles of stones.  There are an even number of piles **arranged in a row**, and each pile has a positive integer number of stones ```piles[i]```.

The objective of the game is to end with the most stones.  The total number of stones is odd, so there are no ties.

Alex and Lee take turns, with Alex starting first.  Each turn, a player takes the entire pile of stones from either the beginning or the end of the row.  This continues until there are no more piles left, at which point the person with the most stones wins.

Assuming Alex and Lee play optimally, return ```True``` if and only if Alex wins the game.

#### Example 1:
<pre>
<strong>Input:</strong> [5,3,4,5]
<strong>Output:</strong> true
<strong>Explanation:</strong>
Alex starts first, and can only take the first 5 or the last 5.
Say he takes the first 5, so that the row becomes [3, 4, 5].
If Lee takes 3, then the board is [4, 5], and Alex takes 5 to win with 10 points.
If Lee takes the last 5, then the board is [3, 4], and Alex takes 4 to win with 9 points.
This demonstrated that taking the first 5 was a winning move for Alex, so we return true.
</pre>

#### Note:
1. ```2 <= piles.length <= 500```
2. ```piles.length``` is even.
3. ```1 <= piles[i] <= 500```
4. ```sum(piles)``` is odd.

## Solutions (Rust)

### 1. Dynamic Programming
```Rust
impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let mut dp = vec![vec![0; piles.len()]; piles.len()];

        for r in 1..piles.len() {
            for l in (0..r).rev() {
                dp[l][r] = match (r - l) % 2 {
                    1 => (piles[l] + dp[l + 1][r]).max(piles[r] + dp[l][r - 1]),
                    _ => (dp[l + 1][r]).min(dp[l][r - 1]),
                };
            }
        }

        dp[0].last().unwrap() * 2 > piles.iter().sum()
    }
}
```

### 2. Mathematical
```Rust
impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        true
    }
}
```
