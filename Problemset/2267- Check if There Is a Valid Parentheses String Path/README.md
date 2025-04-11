# 2267. Check if There Is a Valid Parentheses String Path
A parentheses string is a **non-empty** string consisting only of `'('` and `')'`. It is **valid** if **any** of the following conditions is **true**:
* It is `()`.
* It can be written as `AB` (`A` concatenated with `B`), where `A` and `B` are valid parentheses strings.
* It can be written as `(A)`, where `A` is a valid parentheses string.

You are given an `m x n` matrix of parentheses `grid`. A **valid parentheses string path** in the grid is a path satisfying **all** of the following conditions:
* The path starts from the upper left cell `(0, 0)`.
* The path ends at the bottom-right cell `(m - 1, n - 1)`.
* The path only ever moves **down** or **right**.
* The resulting parentheses string formed by the path is **valid**.

Return `true` *if there exists a **valid parentheses string path** in the grid*. Otherwise, return `false`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/03/15/example1drawio.png)
<pre>
<strong>Input:</strong> grid = [["(","(","("],[")","(",")"],["(","(",")"],["(","(",")"]]
<strong>Output:</strong> true
<strong>Explanation:</strong> The above diagram shows two possible paths that form valid parentheses strings.
The first path shown results in the valid parentheses string "()(())".
The second path shown results in the valid parentheses string "((()))".
Note that there may be other valid parentheses string paths.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/03/15/example2drawio.png)
<pre>
<strong>Input:</strong> grid = [[")",")"],["(","("]]
<strong>Output:</strong> false
<strong>Explanation:</strong> The two possible paths form the parentheses strings "))(" and ")((". Since neither of them are valid parentheses strings, we return false.
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 100`
* `grid[i][j]` is either `'('` or `')'`.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut stack = vec![(0, 0, 1)];
        let mut visited = HashSet::from([(0, 0, 1)]);

        if grid[0][0] == ')' || grid[m - 1][n - 1] == '(' || (m + n) % 2 == 0 {
            return false;
        }

        while let Some((i, j, diff)) = stack.pop() {
            if i == m - 1 && j == n - 1 && diff == 0 {
                return true;
            }

            if i + 1 < m {
                let new_diff = diff + 81 - grid[i + 1][j] as i32 * 2;
                if new_diff >= 0 && !visited.contains(&(i + 1, j, new_diff)) {
                    stack.push((i + 1, j, new_diff));
                    visited.insert((i + 1, j, new_diff));
                }
            }

            if j + 1 < n {
                let new_diff = diff + 81 - grid[i][j + 1] as i32 * 2;
                if new_diff >= 0 && !visited.contains(&(i, j + 1, new_diff)) {
                    stack.push((i, j + 1, new_diff));
                    visited.insert((i, j + 1, new_diff));
                }
            }
        }

        false
    }
}
```
