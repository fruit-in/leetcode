# 1496. Path Crossing
Given a string `path`, where `path[i] = 'N'`, `'S'`, `'E'` or `'W'`, each representing moving one unit north, south, east, or west, respectively. You start at the origin `(0, 0)` on a 2D plane and walk on the path specified by `path`.

Return `True` if the path crosses itself at any point, that is, if at any time you are on a location you've previously visited. Return `False` otherwise.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/06/10/screen-shot-2020-06-10-at-123929-pm.png)
<pre>
<strong>Input:</strong> path = "NES"
<strong>Output:</strong> false
<strong>Explanation:</strong> Notice that the path doesn't cross any point more than once.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/06/10/screen-shot-2020-06-10-at-123843-pm.png)
<pre>
<strong>Input:</strong> path = "NESWW"
<strong>Output:</strong> true
<strong>Explanation:</strong> Notice that the path visits the origin twice.
</pre>

#### Constraints:
* `1 <= path.length <= 10^4`
* `path` will only consist of characters in `{'N', 'S', 'E', 'W}`

## Solutions (Rust)

### 1. Set
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut visited = HashSet::new();
        let mut x = 0;
        let mut y = 0;
        visited.insert((x, y));

        for dir in path.bytes() {
            match dir {
                b'N' => y += 1,
                b'S' => y -= 1,
                b'E' => x += 1,
                _ => x -= 1,
            };
            if !visited.insert((x, y)) {
                return true;
            }
        }

        false
    }
}
```
