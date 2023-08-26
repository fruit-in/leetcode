# 675. Cut Off Trees for Golf Event
You are asked to cut off all the trees in a forest for a golf event. The forest is represented as an `m x n` matrix. In this matrix:

* `0` means the cell cannot be walked through.
* `1` represents an empty cell that can be walked through.
* A number greater than `1` represents a tree in a cell that can be walked through, and this number is the tree's height.

In one step, you can walk in any of the four directions: north, east, south, and west. If you are standing in a cell with a tree, you can choose whether to cut it off.

You must cut off the trees in order from shortest to tallest. When you cut off a tree, the value at its cell becomes `1` (an empty cell).

Starting from the point `(0, 0)`, return *the minimum steps you need to walk to cut off all the trees*. If you cannot cut off all the trees, return `-1`.

**Note:** The input is generated such that no two trees have the same height, and there is at least one tree needs to be cut off.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/11/26/trees1.jpg)
<pre>
<strong>Input:</strong> forest = [[1,2,3],[0,0,4],[7,6,5]]
<strong>Output:</strong> 6
<strong>Explanation:</strong> Following the path above allows you to cut off the trees from shortest to tallest in 6 steps.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/11/26/trees2.jpg)
<pre>
<strong>Input:</strong> forest = [[1,2,3],[0,0,0],[7,6,5]]
<strong>Output:</strong> -1
<strong>Explanation:</strong> The trees in the bottom row cannot be accessed as the middle row is blocked.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> forest = [[2,3,4],[0,0,5],[8,7,6]]
<strong>Output:</strong> 6
<strong>Explanation:</strong> You can follow the same path as Example 1 to cut off all the trees.
Note that you can cut off the first tree at (0, 0) before making any steps.
</pre>

#### Constraints:
* `m == forest.length`
* `n == forest[i].length`
* `1 <= m, n <= 50`
* <code>0 <= forest[i][j] <= 10<sup>9</sup></code>
* Heights of all trees are **distinct**.

## Solutions (Rust)

### 1. Solution
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
