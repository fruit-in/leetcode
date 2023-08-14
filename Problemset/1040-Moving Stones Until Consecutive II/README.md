# 1040. Moving Stones Until Consecutive II
There are some stones in different positions on the X-axis. You are given an integer array `stones`, the positions of the stones.

Call a stone an **endpoint stone** if it has the smallest or largest position. In one move, you pick up an **endpoint stone** and move it to an unoccupied position so that it is no longer an **endpoint stone**.

* In particular, if the stones are at say, `stones = [1,2,5]`, you cannot move the endpoint stone at position `5`, since moving it to any position (such as `0`, or `3`) will still keep that stone as an endpoint stone.

The game ends when you cannot make any more moves (i.e., the stones are in three consecutive positions).

Return *an integer array* `answer` *of length* `2` *where*:

* `answer[0]` *is the minimum number of moves you can play, and*
* `answer[1]` *is the maximum number of moves you can play*.

#### Example 1:
<pre>
<strong>Input:</strong> stones = [7,4,9]
<strong>Output:</strong> [1,2]
<strong>Explanation:</strong> We can move 4 -> 8 for one move to finish the game.
Or, we can move 9 -> 5, 4 -> 6 for two moves to finish the game.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> stones = [6,5,4,3,10]
<strong>Output:</strong> [2,3]
<strong>Explanation:</strong> We can move 3 -> 8 then 10 -> 7 to finish the game.
Or, we can move 3 -> 7, 4 -> 8, 5 -> 9 to finish the game.
Notice we cannot move 10 -> 2 to finish the game, because that would be an illegal move.
</pre>

#### Constraints:
* <code>3 <= stones.length <= 10<sup>4</sup></code>
* <code>1 <= stones[i] <= 10<sup>9</sup></code>
* All the values of `stones` are **unique**.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn num_moves_stones_ii(stones: Vec<i32>) -> Vec<i32> {
        let n = stones.len();
        let mut stones = stones;
        let mut i = 0;
        let mut answer = vec![i32::MAX, 0];

        stones.sort_unstable();

        for j in 0..n {
            while i < n && stones[i] <= stones[j] + n as i32 - 1 {
                i += 1;
            }
            answer[0] = answer[0].min((j + n - i) as i32);
        }
        if answer[0] == 1
            && ((stones[n - 2] - stones[0] + 2 == n as i32 && stones[n - 1] - stones[n - 2] > 2)
                || (stones[n - 1] - stones[1] + 2 == n as i32 && stones[1] - stones[0] > 2))
        {
            answer[0] = 2;
        }

        answer[1] = stones[n - 1] - stones[0] - n as i32 + 2
            - (stones[1] - stones[0]).min(stones[n - 1] - stones[n - 2]);

        answer
    }
}
```
