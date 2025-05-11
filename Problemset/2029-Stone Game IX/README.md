# 2029. Stone Game IX
Alice and Bob continue their games with stones. There is a row of n stones, and each stone has an associated value. You are given an integer array `stones`, where `stones[i]` is the **value** of the <code>i<sup>th</sup></code> stone.

Alice and Bob take turns, with **Alice** starting first. On each turn, the player may remove any stone from `stones`. The player who removes a stone **loses** if the **sum** of the values of **all removed stones** is divisible by `3`. Bob will win automatically if there are no remaining stones (even if it is Alice's turn).

Assuming both players play **optimally**, return `true` *if Alice wins and* `false` *if Bob wins*.

#### Example 1:
<pre>
<strong>Input:</strong> stones = [2,1]
<strong>Output:</strong> true
<strong>Explanation:</strong> The game will be played as follows:
- Turn 1: Alice can remove either stone.
- Turn 2: Bob removes the remaining stone.
The sum of the removed stones is 1 + 2 = 3 and is divisible by 3. Therefore, Bob loses and Alice wins the game.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> stones = [2]
<strong>Output:</strong> false
<strong>Explanation:</strong> Alice will remove the only stone, and the sum of the values on the removed stones is 2.
Since all the stones are removed and the sum of values is not divisible by 3, Bob wins the game.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> stones = [5,1,2,4,3]
<strong>Output:</strong> false
<strong>Explanation:</strong> Bob will always win. One possible way for Bob to win is shown below:
- Turn 1: Alice can remove the second stone with value 1. Sum of removed stones = 1.
- Turn 2: Bob removes the fifth stone with value 3. Sum of removed stones = 1 + 3 = 4.
- Turn 3: Alices removes the fourth stone with value 4. Sum of removed stones = 1 + 3 + 4 = 8.
- Turn 4: Bob removes the third stone with value 2. Sum of removed stones = 1 + 3 + 4 + 2 = 10.
- Turn 5: Alice removes the first stone with value 5. Sum of removed stones = 1 + 3 + 4 + 2 + 5 = 15.
Alice loses the game because the sum of the removed stones (15) is divisible by 3. Bob wins the game.
</pre>

#### Constraints:
* <code>1 <= stones.length <= 10<sup>5</sup></code>
* <code>1 <= stones[i] <= 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        let mut count = [0_i32; 3];

        for &x in &stones {
            count[x as usize % 3] += 1;
        }

        (count[0] % 2 == 0 && count[1].min(count[2]) > 0)
            || (count[0] % 2 == 1 && (count[2] - count[1]).abs() > 2)
    }
}
```
