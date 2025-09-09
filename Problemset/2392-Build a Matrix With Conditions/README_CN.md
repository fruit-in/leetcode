# 2392. 给定条件下构造矩阵
给你一个 **正** 整数 `k` ，同时给你：
* 一个大小为 `n` 的二维整数数组 `rowConditions` ，其中 <code>rowConditions[i] = [above<sub>i</sub>, below<sub>i</sub>]</code> 和
* 一个大小为 `m` 的二维整数数组 `colConditions` ，其中 <code>colConditions[i] = [left<sub>i</sub>, right<sub>i</sub>]</code> 。

两个数组里的整数都是 `1` 到 `k` 之间的数字。

你需要构造一个 `k x k` 的矩阵，`1` 到 `k` 每个数字需要 **恰好出现一次** 。剩余的数字都是 `0` 。

矩阵还需要满足以下条件：
* 对于所有 `0` 到 `n - 1` 之间的下标 `i` ，数字 <code>above<sub>i</sub></code> 所在的 **行** 必须在数字 <code>below<sub>i</sub></code> 所在行的上面。
* 对于所有 `0` 到 `m - 1` 之间的下标 `i` ，数字 <code>left<sub>i</sub></code> 所在的 **列** 必须在数字 <code>right<sub>i</sub></code> 所在列的左边。

返回满足上述要求的 **任意** 矩阵。如果不存在答案，返回一个空的矩阵。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/07/06/gridosdrawio.png)
<pre>
<strong>输入:</strong> k = 3, rowConditions = [[1,2],[3,2]], colConditions = [[2,1],[3,2]]
<strong>输出:</strong> [[3,0,0],[0,0,1],[0,2,0]]
<strong>解释:</strong> 上图为一个符合所有条件的矩阵。
行要求如下：
- 数字 1 在第 1 行，数字 2 在第 2 行，1 在 2 的上面。
- 数字 3 在第 0 行，数字 2 在第 2 行，3 在 2 的上面。
列要求如下：
- 数字 2 在第 1 列，数字 1 在第 2 列，2 在 1 的左边。
- 数字 3 在第 0 列，数字 2 在第 1 列，3 在 2 的左边。
注意，可能有多种正确的答案。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> k = 3, rowConditions = [[1,2],[2,3],[3,1],[2,3]], colConditions = [[2,1]]
<strong>输出:</strong> []
<strong>解释:</strong> 由前两个条件可以得到 3 在 1 的下面，但第三个条件是 3 在 1 的上面。
没有符合条件的矩阵存在，所以我们返回空矩阵。
</pre>

#### 提示:
* `2 <= k <= 400`
* <code>1 <= rowConditions.length, colConditions.length <= 10<sup>4</sup></code>
* `rowConditions[i].length == colConditions[i].length == 2`
* <code>1 <= above<sub>i</sub>, below<sub>i</sub>, left<sub>i</sub>, right<sub>i</sub> <= k</code>
* <code>above<sub>i</sub> != below<sub>i</sub></code>
* <code>left<sub>i</sub> != right<sub>i</sub></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn build_matrix(
        k: i32,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut belows = vec![vec![]; k + 1];
        let mut rights = vec![vec![]; k + 1];
        let mut indegree = vec![0; k + 1];
        let mut stack = vec![];
        let mut r = 0;
        let mut c = 0;
        let mut row = vec![k; k + 1];
        let mut col = vec![k; k + 1];
        let mut ret = vec![vec![0; k]; k];

        for condition in &row_conditions {
            let (above, below) = (condition[0] as usize, condition[1] as usize);
            belows[above].push(below);
            indegree[below] += 1;
        }

        stack = (1..=k).filter(|&x| indegree[x] == 0).collect();

        while let Some(above) = stack.pop() {
            row[above] = r;
            r += 1;
            for &below in &belows[above] {
                indegree[below] -= 1;
                if indegree[below] == 0 {
                    stack.push(below);
                }
            }
        }

        if r != k {
            return vec![];
        }

        indegree = vec![0; k + 1];

        for condition in &col_conditions {
            let (left, right) = (condition[0] as usize, condition[1] as usize);
            rights[left].push(right);
            indegree[right] += 1;
        }

        stack = (1..=k).filter(|&x| indegree[x] == 0).collect();

        while let Some(left) = stack.pop() {
            col[left] = c;
            c += 1;
            for &right in &rights[left] {
                indegree[right] -= 1;
                if indegree[right] == 0 {
                    stack.push(right);
                }
            }
        }

        if c != k {
            return vec![];
        }

        for x in 1..=k {
            ret[row[x]][col[x]] = x as i32;
        }

        ret
    }
}
```
