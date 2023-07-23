# 2120. Execution of All Suffix Instructions Staying in a Grid
There is an `n x n` grid, with the top-left cell at `(0, 0)` and the bottom-right cell at `(n - 1, n - 1)`. You are given the integer `n` and an integer array `startPos` where <code>startPos = [start<sub>row</sub>, start<sub>col</sub>]</code> indicates that a robot is initially at cell <code>(start<sub>row</sub>, start<sub>col</sub>)</code>.

You are also given a **0-indexed** string `s` of length `m` where `s[i]` is the <code>i<sup>th</sup></code> instruction for the robot: `'L'` (move left), `'R'` (move right), `'U'` (move up), and `'D'` (move down).

The robot can begin executing from any <code>i<sup>th</sup></code> instruction in `s`. It executes the instructions one by one towards the end of `s` but it stops if either of these conditions is met:

* The next instruction will move the robot off the grid.
* There are no more instructions left to execute.

Return *an array* `answer` *of length* `m` *where* `answer[i]` *is **the number of instructions** the robot can execute if the robot **begins executing from** the* <code>i<sup>th</sup></code> *instruction in* `s`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/12/09/1.png)
<pre>
<strong>Input:</strong> n = 3, startPos = [0,1], s = "RRDDLU"
<strong>Output:</strong> [1,5,4,3,1,0]
<strong>Explanation:</strong> Starting from startPos and beginning execution from the ith instruction:
- 0th: "RRDDLU". Only one instruction "R" can be executed before it moves off the grid.
- 1st:  "RDDLU". All five instructions can be executed while it stays in the grid and ends at (1, 1).
- 2nd:   "DDLU". All four instructions can be executed while it stays in the grid and ends at (1, 0).
- 3rd:    "DLU". All three instructions can be executed while it stays in the grid and ends at (0, 0).
- 4th:     "LU". Only one instruction "L" can be executed before it moves off the grid.
- 5th:      "U". If moving up, it would move off the grid.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/12/09/2.png)
<pre>
<strong>Input:</strong> n = 2, startPos = [1,1], s = "LURD"
<strong>Output:</strong> [4,1,0,0]
<strong>Explanation:</strong>
- 0th: "LURD".
- 1st:  "URD".
- 2nd:   "RD".
- 3rd:    "D".
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/12/09/3.png)
<pre>
<strong>Input:</strong> n = 1, startPos = [0,0], s = "LRUD"
<strong>Output:</strong> [0,0,0,0]
<strong>Explanation:</strong> No matter which instruction the robot begins execution from, it would move off the grid.
</pre>

#### Constraints:
* `m == s.length`
* `1 <= n, m <= 500`
* `startPos.length == 2`
* <code>0 <= start<sub>row</sub>, start<sub>col</sub> < n</code>
* `s` consists of `'L'`, `'R'`, `'U'`, and `'D'`.

## Solutions (Rust)

### 1. Solution
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
