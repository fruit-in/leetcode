# 529. 扫雷游戏
让我们一起来玩扫雷游戏！

给定一个代表游戏板的二维字符矩阵。 **'M'** 代表一个**未挖出的**地雷，**'E'** 代表一个**未挖出的**空方块，**'B'** 代表没有相邻（上，下，左，右，和所有4个对角线）地雷的**已挖出的**空白方块，**数字**（'1' 到 '8'）表示有多少地雷与这块**已挖出的**方块相邻，**'X'** 则表示一个**已挖出的**地雷。

现在给出在所有**未挖出的**方块中（'M'或者'E'）的下一个点击位置（行和列索引），根据以下规则，返回相应位置被点击后对应的面板：
1. 如果一个地雷（'M'）被挖出，游戏就结束了- 把它改为 **'X'**。
2. 如果一个**没有相邻地雷**的空方块（'E'）被挖出，修改它为（'B'），并且所有和其相邻的方块都应该被递归地揭露。
3. 如果一个**至少与一个地雷相邻**的空方块（'E'）被挖出，修改它为数字（'1'到'8'），表示相邻地雷的数量。
4. 如果在此次点击中，若无更多方块可被揭露，则返回面板。

#### 示例 1:
<pre>
<strong>输入:</strong>
[['E', 'E', 'E', 'E', 'E'],
 ['E', 'E', 'M', 'E', 'E'],
 ['E', 'E', 'E', 'E', 'E'],
 ['E', 'E', 'E', 'E', 'E']]

Click : [3,0]
<strong>输出:</strong>
[['B', '1', 'E', '1', 'B'],
 ['B', '1', 'M', '1', 'B'],
 ['B', '1', '1', '1', 'B'],
 ['B', 'B', 'B', 'B', 'B']]
<strong>解释:</strong>
<img src='https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/12/minesweeper_example_1.png'>
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong>
[['B', '1', 'E', '1', 'B'],
 ['B', '1', 'M', '1', 'B'],
 ['B', '1', '1', '1', 'B'],
 ['B', 'B', 'B', 'B', 'B']]

Click : [1,2]
<strong>输出:</strong>
[['B', '1', 'E', '1', 'B'],
 ['B', '1', 'X', '1', 'B'],
 ['B', '1', '1', '1', 'B'],
 ['B', 'B', 'B', 'B', 'B']]
<strong>解释:</strong>
<img src='https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/12/minesweeper_example_2.png'>
</pre>

#### 注意:
1. 输入矩阵的宽和高的范围为 [1,50]。
2. 点击的位置只能是未被挖出的方块 ('M' 或者 'E')，这也意味着面板至少包含一个可点击的方块。
3. 输入面板不会是游戏结束的状态（即有地雷已被挖出）。
4. 简单起见，未提及的规则在这个问题中可被忽略。例如，当游戏结束时你不需要挖出所有地雷，考虑所有你可能赢得游戏或标记方块的情况。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn update_board(board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        let mut board = board;
        let mut clicks = vec![click];
        let h = board.len() as i32;
        let w = board[0].len() as i32;

        while let Some(click) = clicks.pop() {
            let r = click[0];
            let c = click[1];

            match board[r as usize][c as usize] {
                'M' => board[r as usize][c as usize] = 'X',
                'E' => {
                    let mut mines = 0;
                    let mut empties = Vec::new();

                    for i in (r - 1).max(0)..(r + 2).min(h) {
                        for j in (c - 1).max(0)..(c + 2).min(w) {
                            if (i != r || j != c) {
                                match board[i as usize][j as usize] {
                                    'M' => mines += 1,
                                    'E' => empties.push(vec![i, j]),
                                    _ => (),
                                }
                            }
                        }
                    }

                    board[r as usize][c as usize] = match mines {
                        0 => {
                            clicks.append(&mut empties);
                            'B'
                        },
                        m => (m as u8 + b'0') as char,
                    }
                },
                _ => (),
            }
        }

        board
    }
}
```
