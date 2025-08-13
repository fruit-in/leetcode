# 1036. 逃离大迷宫
在一个 10<sup>6</sup> x 10<sup>6</sup> 的网格中，每个网格上方格的坐标为 `(x, y)` 。

现在从源方格 <code>source = [s<sub>x</sub>, s<sub>y</sub>]</code> 开始出发，意图赶往目标方格 <code>target = [t<sub>x</sub>, t<sub>y</sub>]</code> 。数组 `blocked` 是封锁的方格列表，其中每个 <code>blocked[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> 表示坐标为 <code>(x<sub>i</sub>, y<sub>i</sub>)</code> 的方格是禁止通行的。

每次移动，都可以走到网格中在四个方向上相邻的方格，只要该方格 **不** 在给出的封锁列表 `blocked` 上。同时，不允许走出网格。

只有在可以通过一系列的移动从源方格 `source` 到达目标方格 `target` 时才返回 `true`。否则，返回 `false`。

#### 示例 1:
<pre>
<strong>输入:</strong> blocked = [[0,1],[1,0]], source = [0,0], target = [0,2]
<strong>输出:</strong> false
<strong>解释:</strong>
从源方格无法到达目标方格，因为我们无法在网格中移动。
无法向北或者向东移动是因为方格禁止通行。
无法向南或者向西移动是因为不能走出网格。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> blocked = [], source = [0,0], target = [999999,999999]
<strong>输出:</strong> true
<strong>解释:</strong>
因为没有方格被封锁，所以一定可以到达目标方格。
</pre>

#### 提示:
* `0 <= blocked.length <= 200`
* `blocked[i].length == 2`
* <code>0 <= x<sub>i</sub>, y<sub>i</sub> < 10<sup>6</sup></code>
* `source.length == target.length == 2`
* <code>0 <= s<sub>x</sub>, s<sub>y</sub>, t<sub>x</sub>, t<sub>y</sub> < 10<sup>6</sup></code>
* `source != target`
* 题目数据保证 `source` 和 `target` 不在封锁列表内

## 题解 (Rust)

### 1. 题解
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
