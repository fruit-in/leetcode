# 794. 有效的井字游戏
用字符串数组作为井字游戏的游戏板 `board`。当且仅当在井字游戏过程中，玩家有可能将字符放置成游戏板所显示的状态时，才返回 true。

该游戏板是一个 3 x 3 数组，由字符 `" "`，`"X"` 和 `"O"` 组成。字符 `" "` 代表一个空位。

以下是井字游戏的规则：
* 玩家轮流将字符放入空位（" "）中。
* 第一个玩家总是放字符 “X”，且第二个玩家总是放字符 “O”。
* “X” 和 “O” 只允许放置在空位中，不允许对已放有字符的位置进行填充。
* 当有 3 个相同（且非空）的字符填充任何行、列或对角线时，游戏结束。
* 当所有位置非空时，也算为游戏结束。
* 如果游戏结束，玩家不允许再放置字符。

#### 示例 1:
<pre>
<strong>输入:</strong> board = ["O  ","   ","   "]
<strong>输出:</strong> false
<strong>解释:</strong> 第一个玩家总是放置“X”。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> board = ["XOX"," X ","   "]
<strong>输出:</strong> false
<strong>解释:</strong> 玩家应该是轮流放置的。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> board = ["XXX","   ","OOO"]
<strong>输出:</strong> false
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> board = ["XOX","O O","XOX"]
<strong>输出:</strong> true
</pre>

#### 说明:
* 游戏板 `board` 是长度为 3 的字符串数组，其中每个字符串 `board[i]` 的长度为 3。
* `board[i][j]` 是集合 `{" ", "X", "O"}` 中的一个字符。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        let board = board.concat().into_bytes();
        let x_count = board.iter().filter(|&&c| c == b'X').count();
        let o_count = board.iter().filter(|&&c| c == b'O').count();

        (x_count == o_count + 1 && !Self::is_win(b'O', &board))
            || (x_count == o_count && !Self::is_win(b'X', &board))
    }

    fn is_win(player: u8, board: &[u8]) -> bool {
        [
            (0, 1, 2),
            (3, 4, 5),
            (6, 7, 8),
            (0, 3, 6),
            (1, 4, 7),
            (2, 5, 8),
            (0, 4, 8),
            (2, 4, 6),
        ]
        .iter()
        .any(|&(x, y, z)| [board[x], board[y], board[z]] == [player, player, player])
    }
}
```
