# 1349. Maximum Students Taking Exam
Given a `m * n` matrix `seats` that represent seats distributions in a classroom. If a seat is broken, it is denoted by `'#'` character otherwise it is denoted by a `'.'` character.

Students can see the answers of those sitting next to the left, right, upper left and upper right, but he cannot see the answers of the student sitting directly in front or behind him. Return the **maximum** number of students that can take the exam together without any cheating being possible.

Students must be placed in seats in good condition.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/01/29/image.png)
<pre>
<strong>Input:</strong> seats = [["#",".","#","#",".","#"],
                [".","#","#","#","#","."],
                ["#",".","#","#",".","#"]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> Teacher can place 4 students in available seats so they don't cheat on the exam.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> seats = [[".","#"],
                ["#","#"],
                ["#","."],
                ["#","#"],
                [".","#"]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> Place all students in available seats.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> seats = [["#",".",".",".","#"],
                [".","#",".","#","."],
                [".",".","#",".","."],
                [".","#",".","#","."],
                ["#",".",".",".","#"]]
<strong>Output:</strong> 10
<strong>Explanation:</strong> Place students in available seats in column 1, 3 and 5.
</pre>

#### Constraints:
* `seats` contains only characters `'.' and'#'`.
* `m == seats.length`
* `n == seats[i].length`
* `1 <= m <= 8`
* `1 <= n <= 8`

## Solutions (Rust)

### 1. Solution
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
