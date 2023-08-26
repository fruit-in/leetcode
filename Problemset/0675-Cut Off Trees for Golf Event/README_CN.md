# 675. 为高尔夫比赛砍树
你被请来给一个要举办高尔夫比赛的树林砍树。树林由一个 `m x n` 的矩阵表示， 在这个矩阵中：

* `0` 表示障碍，无法触碰
* `1` 表示地面，可以行走
* `比 1 大的数` 表示有树的单元格，可以行走，数值表示树的高度

每一步，你都可以向上、下、左、右四个方向之一移动一个单位，如果你站的地方有一棵树，那么你可以决定是否要砍倒它。

你需要按照树的高度从低向高砍掉所有的树，每砍过一颗树，该单元格的值变为 `1`（即变为地面）。

你将从 `(0, 0)` 点开始工作，返回你砍完所有树需要走的最小步数。 如果你无法砍完所有的树，返回 `-1` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/11/26/trees1.jpg)
<pre>
<strong>输入:</strong> forest = [[1,2,3],[0,0,4],[7,6,5]]
<strong>输出:</strong> 6
<strong>解释:</strong> 沿着上面的路径，你可以用 6 步，按从最矮到最高的顺序砍掉这些树。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/11/26/trees2.jpg)
<pre>
<strong>输入:</strong> forest = [[1,2,3],[0,0,0],[7,6,5]]
<strong>输出:</strong> -1
<strong>解释:</strong> 由于中间一行被障碍阻塞，无法访问最下面一行中的树。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> forest = [[2,3,4],[0,0,5],[8,7,6]]
<strong>输出:</strong> 6
<strong>解释:</strong> 可以按与示例 1 相同的路径来砍掉所有的树。
(0,0) 位置的树，可以直接砍去，不用算步数。
</pre>

#### 提示:
* `m == forest.length`
* `n == forest[i].length`
* `1 <= m, n <= 50`
* <code>0 <= forest[i][j] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        let m = forest.len();
        let n = forest[0].len();
        let mut trees = vec![];
        let mut seen = HashSet::new();
        let mut cells = VecDeque::new();
        let mut source = (0, 0);
        let mut ret = 0;

        for i in 0..m {
            for j in 0..n {
                if forest[i][j] > 1 {
                    trees.push((i, j));
                }
            }
        }
        trees.sort_unstable_by_key(|&(i, j)| forest[i][j]);

        for &target in &trees {
            seen.clear();
            cells.clear();
            seen.insert(source);
            cells.push_back((source, 0));

            while let Some(((i, j), s)) = cells.pop_front() {
                source = (i, j);

                if source == target {
                    ret += s;
                    break;
                }

                if i > 0 && forest[i - 1][j] != 0 && !seen.contains(&(i - 1, j)) {
                    seen.insert((i - 1, j));
                    cells.push_back(((i - 1, j), s + 1));
                }
                if j > 0 && forest[i][j - 1] != 0 && !seen.contains(&(i, j - 1)) {
                    seen.insert((i, j - 1));
                    cells.push_back(((i, j - 1), s + 1));
                }
                if i + 1 < m && forest[i + 1][j] != 0 && !seen.contains(&(i + 1, j)) {
                    seen.insert((i + 1, j));
                    cells.push_back(((i + 1, j), s + 1));
                }
                if j + 1 < n && forest[i][j + 1] != 0 && !seen.contains(&(i, j + 1)) {
                    seen.insert((i, j + 1));
                    cells.push_back(((i, j + 1), s + 1));
                }
            }

            if source != target {
                return -1;
            }
        }

        ret
    }
}
```
