# 292. Nim Game
You are playing the following Nim Game with your friend: There is a heap of stones on the table, each time one of you take turns to remove 1 to 3 stones. The one who removes the last stone will be the winner. You will take the first turn to remove the stones.

Both of you are very clever and have optimal strategies for the game. Write a function to determine whether you can win the game given the number of stones in the heap.

#### Example 1:
<pre>
<strong>Input:</strong> 4
<strong>Output:</strong> false
<strong>Explanation:</strong> 
If there are 4 stones in the heap, then you will never win the game;
No matter 1, 2, or 3 stones you remove, the last stone will always be removed by your friend.
</pre>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}
```
