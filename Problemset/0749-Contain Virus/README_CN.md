# 749. 隔离病毒
病毒扩散得很快，现在你的任务是尽可能地通过安装防火墙来隔离病毒。

假设世界由 `m x n` 的二维矩阵 `isInfected` 组成， `isInfected[i][j] == 0` 表示该区域未感染病毒，而  `isInfected[i][j] == 1` 表示该区域已感染病毒。可以在任意 2 个相邻单元之间的共享边界上安装一个防火墙（并且只有一个防火墙）。

每天晚上，病毒会从被感染区域向相邻未感染区域扩散，除非被防火墙隔离。现由于资源有限，每天你只能安装一系列防火墙来隔离其中一个被病毒感染的区域（一个区域或连续的一片区域），且该感染区域对未感染区域的威胁最大且 **保证唯一** 。

你需要努力使得最后有部分区域不被病毒感染，如果可以成功，那么返回需要使用的防火墙个数; 如果无法实现，则返回在世界被病毒全部感染时已安装的防火墙个数。

#### 示例 1:
<pre>
<img src="https://assets.leetcode.com/uploads/2021/06/01/virus11-grid.jpg">
<strong>输入:</strong> isInfected = [[0,1,0,0,0,0,0,1],[0,1,0,0,0,0,0,1],[0,0,0,0,0,0,0,1],[0,0,0,0,0,0,0,0]]
<strong>输出:</strong> 10
<strong>解释:</strong> 一共有两块被病毒感染的区域。
在第一天，添加 5 墙隔离病毒区域的左侧。病毒传播后的状态是:
<img src="https://assets.leetcode.com/uploads/2021/06/01/virus12edited-grid.jpg">
第二天，在右侧添加 5 个墙来隔离病毒区域。此时病毒已经被完全控制住了。
<img src="https://assets.leetcode.com/uploads/2021/06/01/virus13edited-grid.jpg">
</pre>

#### 示例 2:
<pre>
<img src="https://assets.leetcode.com/uploads/2021/06/01/virus2-grid.jpg">
<strong>输入:</strong> isInfected = [[1,1,1],[1,0,1],[1,1,1]]
<strong>输出:</strong> 4
<strong>解释:</strong> 虽然只保存了一个小区域，但却有四面墙。
注意，防火墙只建立在两个不同区域的共享边界上。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> isInfected = [[1,1,1,0,0,0,0,0,0],[1,0,1,0,1,1,1,1,1],[1,1,1,0,0,0,0,0,0]]
<strong>输出:</strong> 13
<strong>解释:</strong> 在隔离右边感染区域后，隔离左边病毒区域只需要 2 个防火墙。
</pre>

#### 提示:
* `m == isInfected.length`
* `n == isInfected[i].length`
* `1 <= m, n <= 50`
* `isInfected[i][j]` is either `0` or `1`.
* 在整个描述的过程中，总有一个相邻的病毒区域，它将在下一轮 **严格地感染更多未受污染的方块**

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn contain_virus(mut is_infected: Vec<Vec<i32>>) -> i32 {
        let m = is_infected.len();
        let n = is_infected[0].len();
        let mut walls = HashSet::new();

        loop {
            let mut visited = HashSet::new();
            let mut spreads = HashMap::new();
            let mut areas = HashMap::new();
            let mut needs = HashMap::new();
            let mut max_threat_level = 0;
            let mut max_threat_area = (0, 0);

            for i in 0..m {
                for j in 0..n {
                    if is_infected[i][j] != 1 || visited.contains(&(i, j)) {
                        continue;
                    }

                    let mut stack = vec![(i, j)];
                    visited.insert((i, j));
                    areas.insert((i, j), vec![(i, j)]);
                    spreads.insert((i, j), HashSet::new());
                    needs.insert((i, j), HashSet::new());

                    while let Some((r, c)) = stack.pop() {
                        if r > 0 {
                            if is_infected[r - 1][c] == 1 && !visited.contains(&(r - 1, c)) {
                                stack.push((r - 1, c));
                                visited.insert((r - 1, c));
                                areas.get_mut(&(i, j)).unwrap().push((r - 1, c));
                            } else if is_infected[r - 1][c] == 0
                                && !walls.contains(&(r - 1, c, 'D'))
                            {
                                spreads.get_mut(&(i, j)).unwrap().insert((r - 1, c));
                                needs.get_mut(&(i, j)).unwrap().insert((r - 1, c, 'D'));
                            }
                        }
                        if r < m - 1 {
                            if is_infected[r + 1][c] == 1 && !visited.contains(&(r + 1, c)) {
                                stack.push((r + 1, c));
                                visited.insert((r + 1, c));
                                areas.get_mut(&(i, j)).unwrap().push((r + 1, c));
                            } else if is_infected[r + 1][c] == 0 && !walls.contains(&(r, c, 'D')) {
                                spreads.get_mut(&(i, j)).unwrap().insert((r + 1, c));
                                needs.get_mut(&(i, j)).unwrap().insert((r, c, 'D'));
                            }
                        }
                        if c > 0 {
                            if is_infected[r][c - 1] == 1 && !visited.contains(&(r, c - 1)) {
                                stack.push((r, c - 1));
                                visited.insert((r, c - 1));
                                areas.get_mut(&(i, j)).unwrap().push((r, c - 1));
                            } else if is_infected[r][c - 1] == 0
                                && !walls.contains(&(r, c - 1, 'R'))
                            {
                                spreads.get_mut(&(i, j)).unwrap().insert((r, c - 1));
                                needs.get_mut(&(i, j)).unwrap().insert((r, c - 1, 'R'));
                            }
                        }
                        if c < n - 1 {
                            if is_infected[r][c + 1] == 1 && !visited.contains(&(r, c + 1)) {
                                stack.push((r, c + 1));
                                visited.insert((r, c + 1));
                                areas.get_mut(&(i, j)).unwrap().push((r, c + 1));
                            } else if is_infected[r][c + 1] == 0 && !walls.contains(&(r, c, 'R')) {
                                spreads.get_mut(&(i, j)).unwrap().insert((r, c + 1));
                                needs.get_mut(&(i, j)).unwrap().insert((r, c, 'R'));
                            }
                        }
                    }

                    if spreads[&(i, j)].len() > max_threat_level {
                        max_threat_level = spreads[&(i, j)].len();
                        max_threat_area = (i, j);
                    }
                }
            }

            if max_threat_level == 0 {
                break;
            }

            for (&(r, c), s) in spreads.iter() {
                if (r, c) != max_threat_area {
                    for &(r, c) in s {
                        is_infected[r][c] = 1;
                    }
                }
            }

            for &(r, c) in &areas[&max_threat_area] {
                is_infected[r][c] = -1;
            }

            for &(r, c, d) in &needs[&max_threat_area] {
                walls.insert((r, c, d));
            }
        }

        walls.len() as i32
    }
}
```
