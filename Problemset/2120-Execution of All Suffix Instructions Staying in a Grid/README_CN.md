# 2120. 执行所有后缀指令
现有一个 `n x n` 大小的网格，左上角单元格坐标 `(0, 0)` ，右下角单元格坐标 `(n - 1, n - 1)` 。给你整数 `n` 和一个整数数组 `startPos` ，其中 <code>startPos = [start<sub>row</sub>, start<sub>col</sub>]</code> 表示机器人最开始在坐标为 <code>(start<sub>row</sub>, start<sub>col</sub>)</code> 的单元格上。

另给你一个长度为 `m` 、下标从 **0** 开始的字符串 `s` ，其中 `s[i]` 是对机器人的第 `i` 条指令：`'L'`（向左移动），`'R'`（向右移动），`'U'`（向上移动）和 `'D'`（向下移动）。

机器人可以从 `s` 中的任一第 `i` 条指令开始执行。它将会逐条执行指令直到 `s` 的末尾，但在满足下述条件之一时，机器人将会停止：

* 下一条指令将会导致机器人移动到网格外。
* 没有指令可以执行。

返回一个长度为 `m` 的数组 `answer` ，其中 `answer[i]` 是机器人从第 `i` 条指令 **开始** ，可以执行的 **指令数目** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/12/09/1.png)
<pre>
<strong>输入:</strong> n = 3, startPos = [0,1], s = "RRDDLU"
<strong>输出:</strong> [1,5,4,3,1,0]
<strong>解释:</strong> 机器人从 startPos 出发，并从第 i 条指令开始执行：
- 0: "RRDDLU" 在移动到网格外之前，只能执行一条 "R" 指令。
- 1:  "RDDLU" 可以执行全部五条指令，机器人仍在网格内，最终到达 (0, 0) 。
- 2:   "DDLU" 可以执行全部四条指令，机器人仍在网格内，最终到达 (0, 0) 。
- 3:    "DLU" 可以执行全部三条指令，机器人仍在网格内，最终到达 (0, 0) 。
- 4:     "LU" 在移动到网格外之前，只能执行一条 "L" 指令。
- 5:      "U" 如果向上移动，将会移动到网格外。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/12/09/2.png)
<pre>
<strong>输入:</strong> n = 2, startPos = [1,1], s = "LURD"
<strong>输出:</strong> [4,1,0,0]
<strong>解释:</strong>
- 0: "LURD"
- 1:  "URD"
- 2:   "RD"
- 3:    "D"
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/12/09/3.png)
<pre>
<strong>输入:</strong> n = 1, startPos = [0,0], s = "LRUD"
<strong>输出:</strong> [0,0,0,0]
<strong>解释:</strong> 无论机器人从哪条指令开始执行，都会移动到网格外。
</pre>

#### 提示:
* `m == s.length`
* `1 <= n, m <= 500`
* `startPos.length == 2`
* <code>0 <= start<sub>row</sub>, start<sub>col</sub> < n</code>
* `s` 由 `'L'`、`'R'`、`'U'` 和 `'D'` 组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32> {
        let s = s.as_bytes();
        let mut answer = vec![0; s.len()];

        for i in 0..s.len() {
            let mut pos = (start_pos[0], start_pos[1]);

            for j in i..s.len() {
                if s[j] == b'L' && pos.1 > 0 {
                    pos.1 -= 1;
                } else if s[j] == b'R' && pos.1 < n - 1 {
                    pos.1 += 1;
                } else if s[j] == b'U' && pos.0 > 0 {
                    pos.0 -= 1;
                } else if s[j] == b'D' && pos.0 < n - 1 {
                    pos.0 += 1;
                } else {
                    break;
                }

                answer[i] += 1;
            }
        }

        answer
    }
}
```
