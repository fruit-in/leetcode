# 1349. 参加考试的最大学生数
给你一个 `m * n` 的矩阵 `seats` 表示教室中的座位分布。如果座位是坏的（不可用），就用 `'#'` 表示；否则，用 `'.'` 表示。

学生可以看到左侧、右侧、左上、右上这四个方向上紧邻他的学生的答卷，但是看不到直接坐在他前面或者后面的学生的答卷。请你计算并返回该考场可以容纳的同时参加考试且无法作弊的 **最大** 学生人数。

学生必须坐在状况良好的座位上。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/01/29/image.png)
<pre>
<strong>输入:</strong> seats = [["#",".","#","#",".","#"],
                [".","#","#","#","#","."],
                ["#",".","#","#",".","#"]]
<strong>输出:</strong> 4
<strong>解释:</strong> 教师可以让 4 个学生坐在可用的座位上，这样他们就无法在考试中作弊。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> seats = [[".","#"],
                ["#","#"],
                ["#","."],
                ["#","#"],
                [".","#"]]
<strong>输出:</strong> 3
<strong>解释:</strong> 让所有学生坐在可用的座位上。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> seats = [["#",".",".",".","#"],
                [".","#",".","#","."],
                [".",".","#",".","."],
                [".","#",".","#","."],
                ["#",".",".",".","#"]]
<strong>输出:</strong> 10
<strong>解释:</strong> 让学生坐在第 1、3 和 5 列的可用座位上。
</pre>

#### 提示:
* `seats` 只包含字符 `'.' 和'#'`
* `m == seats.length`
* `n == seats[i].length`
* `1 <= m <= 8`
* `1 <= n <= 8`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        let (m, n) = (seats.len(), seats[0].len());
        let mut prev = HashMap::from([(0, 0)]);

        for i in 0..m {
            let mut seats_mask = 0;
            let mut tmp = HashMap::new();

            for j in 0..n {
                if seats[i][j] == '.' {
                    seats_mask |= 1 << j;
                }
            }

            for students_mask in 0_i32..1 << n {
                if students_mask | seats_mask != seats_mask
                    || (0..n).any(|i| (students_mask >> i) & 3 == 3)
                {
                    continue;
                }

                for (&prev_students_mask, &students) in prev.iter() {
                    if (0..n).all(|i| ((students_mask | prev_students_mask) >> i) & 3 != 3)
                        && *tmp.get(&students_mask).unwrap_or(&0)
                            <= students_mask.count_ones() + students
                    {
                        tmp.insert(students_mask, students_mask.count_ones() + students);
                    }
                }
            }

            prev = tmp;
        }

        *prev.values().max().unwrap() as i32
    }
}
```
