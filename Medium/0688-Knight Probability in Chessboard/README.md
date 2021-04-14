# 688. Knight Probability in Chessboard
On an `N`x`N` chessboard, a knight starts at the `r`-th row and `c`-th column and attempts to make exactly `K` moves. The rows and columns are 0 indexed, so the top-left square is `(0, 0)`, and the bottom-right square is `(N-1, N-1)`.

A chess knight has 8 possible moves it can make, as illustrated below. Each move is two squares in a cardinal direction, then one square in an orthogonal direction.

![](https://assets.leetcode.com/uploads/2018/10/12/knight.png)

Each time the knight is to move, it chooses one of eight possible moves uniformly at random (even if the piece would go off the chessboard) and moves there.

The knight continues moving until it has made exactly `K` moves or has moved off the chessboard. Return the probability that the knight remains on the board after it has stopped moving.

#### Example:
<pre>
<strong>Input:</strong> 3, 2, 0, 0
<strong>Output:</strong> 0.0625
<strong>Explanation:</strong> There are two moves (to (1,2), (2,1)) that will keep the knight on the board.
From each of those positions, there are also two moves that will keep the knight on the board.
The total probability the knight stays on the board is 0.0625.
</pre>

#### Note:
* `N` will be between 1 and 25.
* `K` will be between 0 and 100.
* The knight always initially starts on the board.

## Solutions (Ruby)

### 1. Dynamic Programming
```Ruby
# @param {Integer} n
# @param {Integer} k
# @param {Integer} r
# @param {Integer} c
# @return {Float}
def knight_probability(n, k, r, c)
  dp = Array.new(n) { [0] * n }
  dp[r][c] = 1

  (1..k).each do |_|
    dp_ = Array.new(n) { [0] * n }
    (0...n).each do |r|
      (0...n).each do |c|
        [[-2, 1], [-1, 2], [1, 2], [2, 1], [2, -1], [1, -2], [-1, -2], [-2, -1]].each do |dr, dc|
          dp_[r][c] += probability(dp, n, r + dr, c + dc) / 8.0
        end
      end
    end
    dp = dp_
  end

  dp.flatten.sum
end

# @param {Integer[][]} dp
# @param {Integer} n
# @param {Integer} r
# @param {Integer} c
# @return {Float}
def probability(dp, n, r, c)
  r < 0 || c < 0 || r >= n || c >= n ? 0.0 : dp[r][c]
end
```

## Solutions (Rust)

### 1. Dynamic Programming
```Rust
impl Solution {
    pub fn knight_probability(n: i32, k: i32, r: i32, c: i32) -> f64 {
        let mut dp = vec![vec![0.0; n as usize]; n as usize];
        dp[r as usize][c as usize] = 1.0;

        for _ in 0..k {
            let mut dp_ = vec![vec![0.0; n as usize]; n as usize];
            for r in 0..n {
                for c in 0..n {
                    dp_[r as usize][c as usize] = (Self::probability(&dp, n, r - 2, c + 1)
                        + Self::probability(&dp, n, r - 1, c + 2)
                        + Self::probability(&dp, n, r + 1, c + 2)
                        + Self::probability(&dp, n, r + 2, c + 1)
                        + Self::probability(&dp, n, r + 2, c - 1)
                        + Self::probability(&dp, n, r + 1, c - 2)
                        + Self::probability(&dp, n, r - 1, c - 2)
                        + Self::probability(&dp, n, r - 2, c - 1))
                        / 8.0;
                }
            }
            dp = dp_;
        }

        dp.concat().iter().sum::<f64>()
    }

    fn probability(dp: &[Vec<f64>], n: i32, r: i32, c: i32) -> f64 {
        if r < 0 || c < 0 || r >= n || c >= n {
            0.0
        } else {
            dp[r as usize][c as usize]
        }
    }
}
```
