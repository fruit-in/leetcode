# 1025. Divisor Game
Alice and Bob take turns playing a game, with Alice starting first.

Initially, there is a number <code>N</code> on the chalkboard.  On each player's turn, that player makes a *move* consisting of:
* Choosing any <code>x</code> with <code>0 < x < N</code> and <code>N % x == 0</code>.
* Replacing the number <code>N</code> on the chalkboard with <code>N - x</code>.

Also, if a player cannot make a move, they lose the game.

Return <code>True</code> if and only if Alice wins the game, assuming both players play optimally.

#### Example 1:
<pre>
<strong>Input:</strong> 2
<strong>Output:</strong> true
<strong>Explanation:</strong> Alice chooses 1, and Bob has no more moves.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 3
<strong>Output:</strong> false
<strong>Explanation:</strong> Alice chooses 1, Bob chooses 1, and Alice has no more moves.
</pre>

#### Note:
1. <code>1 <= N <= 1000</code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        n % 2 == 0
    }
}
```
