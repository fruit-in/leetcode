# 1036. Escape a Large Maze
There is a 1 million by 1 million grid on an XY-plane, and the coordinates of each grid square are `(x, y)`.

We start at the <code>source = [s<sub>x</sub>, s<sub>y</sub>]</code> square and want to reach the <code>target = [t<sub>x</sub>, t<sub>y</sub>]</code> square. There is also an array of `blocked` squares, where each <code>blocked[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> represents a blocked square with coordinates <code>(x<sub>i</sub>, y<sub>i</sub>)</code>.

Each move, we can walk one square north, east, south, or west if the square is **not** in the array of `blocked` squares. We are also not allowed to walk outside of the grid.

Return `true` *if and only if it is possible to reach the* `target` *square from the* `source` *square through a sequence of valid moves*.

#### Example 1:
<pre>
<strong>Input:</strong> blocked = [[0,1],[1,0]], source = [0,0], target = [0,2]
<strong>Output:</strong> false
<strong>Explanation:</strong> The target square is inaccessible starting from the source square because we cannot move.
We cannot move north or east because those squares are blocked.
We cannot move south or west because we cannot go outside of the grid.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> blocked = [], source = [0,0], target = [999999,999999]
<strong>Output:</strong> true
<strong>Explanation:</strong> Because there are no blocked cells, it is possible to reach the target square.
</pre>

#### Constraints:
* `0 <= blocked.length <= 200`
* `blocked[i].length == 2`
* <code>0 <= x<sub>i</sub>, y<sub>i</sub> < 10<sup>6</sup></code>
* `source.length == target.length == 2`
* <code>0 <= s<sub>x</sub>, s<sub>y</sub>, t<sub>x</sub>, t<sub>y</sub> < 10<sup>6</sup></code>
* `source != target`
* It is guaranteed that `source` and `target` are not blocked.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        let mut deque = VecDeque::from([(source[0], source[1], 0)]);
        let mut visited = blocked.iter().map(|b| (b[0], b[1])).collect::<HashSet<_>>();
        visited.insert((source[0], source[1]));

        while let Some((x, y, steps)) = deque.pop_front() {
            if x == target[0] && y == target[1] {
                return true;
            }

            if steps >= blocked.len() * 2 {
                deque.push_back((0, 0, 0));
                break;
            }

            if x > 0 && !visited.contains(&(x - 1, y)) {
                deque.push_back((x - 1, y, steps + 1));
                visited.insert((x - 1, y));
            }
            if y > 0 && !visited.contains(&(x, y - 1)) {
                deque.push_back((x, y - 1, steps + 1));
                visited.insert((x, y - 1));
            }
            if x < 999999 && !visited.contains(&(x + 1, y)) {
                deque.push_back((x + 1, y, steps + 1));
                visited.insert((x + 1, y));
            }
            if y < 999999 && !visited.contains(&(x, y + 1)) {
                deque.push_back((x, y + 1, steps + 1));
                visited.insert((x, y + 1));
            }
        }

        if deque.is_empty() {
            return false;
        }

        deque = VecDeque::from([(target[0], target[1], 0)]);
        visited = blocked.iter().map(|b| (b[0], b[1])).collect::<HashSet<_>>();
        visited.insert((target[0], target[1]));

        while let Some((x, y, steps)) = deque.pop_front() {
            if steps >= blocked.len() * 2 {
                deque.push_back((0, 0, 0));
                break;
            }

            if x > 0 && !visited.contains(&(x - 1, y)) {
                deque.push_back((x - 1, y, steps + 1));
                visited.insert((x - 1, y));
            }
            if y > 0 && !visited.contains(&(x, y - 1)) {
                deque.push_back((x, y - 1, steps + 1));
                visited.insert((x, y - 1));
            }
            if x < 999999 && !visited.contains(&(x + 1, y)) {
                deque.push_back((x + 1, y, steps + 1));
                visited.insert((x + 1, y));
            }
            if y < 999999 && !visited.contains(&(x, y + 1)) {
                deque.push_back((x, y + 1, steps + 1));
                visited.insert((x, y + 1));
            }
        }

        !deque.is_empty()
    }
}
```
