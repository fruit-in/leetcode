# 1275. 找出井字棋的获胜者
*A* 和 *B* 在一个 *3* x *3* 的网格上玩井字棋。

井字棋游戏的规则如下：
* 玩家轮流将棋子放在空方格 (" ") 上。
* 第一个玩家 A 总是用 "X" 作为棋子，而第二个玩家 B 总是用 "O" 作为棋子。
* "X" 和 "O" 只能放在空方格中，而不能放在已经被占用的方格上。
* 只要有 3 个相同的（非空）棋子排成一条直线（行、列、对角线）时，游戏结束。
* 如果所有方块都放满棋子（不为空），游戏也会结束。
* 游戏结束后，棋子无法再进行任何移动。

给你一个数组 ```moves```，其中每个元素是大小为 ```2``` 的另一个数组（元素分别对应网格的行和列），它按照 *A* 和 *B* 的行动顺序（先 *A* 后 *B*）记录了两人各自的棋子位置。

如果游戏存在获胜者（*A* 或 *B*），就返回该游戏的获胜者；如果游戏以平局结束，则返回 "Draw"；如果仍会有行动（游戏未结束），则返回 "Pending"。

你可以假设 ```moves``` 都 **有效**（遵循井字棋规则），网格最初是空的，*A* 将先行动。

#### 示例 1:
<pre>
<strong>输入:</strong> moves = [[0,0],[2,0],[1,1],[2,1],[2,2]]
<strong>输出:</strong> "A"
<strong>解释:</strong> "A" 获胜，他总是先走。
"X  "    "X  "    "X  "    "X  "    "<strong>X</strong>  "
"   " -> "   " -> " X " -> " X " -> " <strong>X</strong> "
"   "    "O  "    "O  "    "OO "    "OO<strong>X</strong>"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> moves = [[0,0],[1,1],[0,1],[0,2],[1,0],[2,0]]
<strong>输出:</strong> "B"
<strong>解释:</strong> "B" 获胜。
"X  "    "X  "    "XX "    "XXO"    "XXO"    "XX<strong>O</strong>"
"   " -> " O " -> " O " -> " O " -> "XO " -> "X<strong>O</strong> "
"   "    "   "    "   "    "   "    "   "    "<strong>O</strong>  "
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> moves = [[0,0],[1,1],[2,0],[1,0],[1,2],[2,1],[0,1],[0,2],[2,2]]
<strong>输出:</strong> 由于没有办法再行动，游戏以平局结束。
<strong>解释:</strong> The game ends in a draw since there are no moves to make.
"XXO"
"OOX"
"XOX"
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> moves = [[0,0],[1,1]]
<strong>输出:</strong> "Pending"
<strong>解释:</strong> 游戏还没有结束。
"X  "
" O "
"   "
</pre>

#### 提示:
* ```1 <= moves.length <= 9```
* ```1 <= moves.length <= 9```
* ```0 <= moves[i][j] <= 2```
* ```moves``` 里没有重复的元素。
* ```moves``` 遵循井字棋的规则。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut scores = [0; 8];

        for (i, mv) in moves.iter().enumerate() {
            let player = i as i32 % 2 * 2 - 1;
            scores[mv[0] as usize] += player;
            scores[mv[1] as usize + 3] += player;
            if *mv == vec![0, 0] || *mv == vec![1, 1] || *mv == vec![2, 2] {
                scores[6] += player;
            }
            if *mv == vec![0, 2] || *mv == vec![1, 1] || *mv == vec![2, 0] {
                scores[7] += player;
            }
        }

        if scores.contains(&-3) {
            "A".to_string()
        } else if scores.contains(&3) {
            "B".to_string()
        } else if moves.len() == 9 {
            "Draw".to_string()
        } else {
            "Pending".to_string()
        }
    }
}
```
