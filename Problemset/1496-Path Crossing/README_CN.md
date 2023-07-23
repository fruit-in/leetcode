# 1496. 判断路径是否相交
给你一个字符串 `path`，其中 `path[i]` 的值可以是 `'N'`、`'S'`、`'E'` 或者 `'W'`，分别表示向北、向南、向东、向西移动一个单位。

机器人从二维平面上的原点 `(0, 0)` 处开始出发，按 `path` 所指示的路径行走。

如果路径在任何位置上出现相交的情况，也就是走到之前已经走过的位置，请返回 `True` ；否则，返回 `False` 。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/06/28/screen-shot-2020-06-10-at-123929-pm.png)
<pre>
<strong>输入:</strong> path = "NES"
<strong>输出:</strong> false
<strong>解释:</strong> 该路径没有在任何位置相交。
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/06/28/screen-shot-2020-06-10-at-123843-pm.png)
<pre>
<strong>输入:</strong> path = "NESWW"
<strong>输出:</strong> true
<strong>解释:</strong> 该路径经过原点两次。
</pre>

#### 提示:
* `1 <= path.length <= 10^4`
* `path` 仅由 `{'N', 'S', 'E', 'W}` 中的字符组成

## 题解 (Rust)

### 1. 集合
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
