# 2392. Build a Matrix With Conditions
You are given a **positive** integer `k`. You are also given:
* a 2D integer array `rowConditions` of size `n` where <code>rowConditions[i] = [above<sub>i</sub>, below<sub>i</sub>]</code>, and
* a 2D integer array `colConditions` of size `m` where <code>colConditions[i] = [left<sub>i</sub>, right<sub>i</sub>]</code>.

The two arrays contain integers from `1` to `k`.

You have to build a `k x k` matrix that contains each of the numbers from `1` to `k` **exactly once**. The remaining cells should have the value `0`.

The matrix should also satisfy the following conditions:
* The number <code>above<sub>i</sub></code> should appear in a **row** that is strictly **above** the row at which the number <code>below<sub>i</sub></code> appears for all `i` from `0` to `n - 1`.
* The number <code>left<sub>i</sub></code> should appear in a **column** that is strictly **left** of the column at which the number <code>right<sub>i</sub></code> appears for all `i` from `0` to `m - 1`.

Return ***any** matrix that satisfies the conditions*. If no answer exists, return an empty matrix.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/07/06/gridosdrawio.png)
<pre>
<strong>Input:</strong> k = 3, rowConditions = [[1,2],[3,2]], colConditions = [[2,1],[3,2]]
<strong>Output:</strong> [[3,0,0],[0,0,1],[0,2,0]]
<strong>Explanation:</strong> The diagram above shows a valid example of a matrix that satisfies all the conditions.
The row conditions are the following:
- Number 1 is in row 1, and number 2 is in row 2, so 1 is above 2 in the matrix.
- Number 3 is in row 0, and number 2 is in row 2, so 3 is above 2 in the matrix.
The column conditions are the following:
- Number 2 is in column 1, and number 1 is in column 2, so 2 is left of 1 in the matrix.
- Number 3 is in column 0, and number 2 is in column 1, so 3 is left of 2 in the matrix.
Note that there may be multiple correct answers.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> k = 3, rowConditions = [[1,2],[2,3],[3,1],[2,3]], colConditions = [[2,1]]
<strong>Output:</strong> []
<strong>Explanation:</strong> From the first two conditions, 3 has to be below 1 but the third conditions needs 3 to be above 1 to be satisfied.
No matrix can satisfy all the conditions, so we return the empty matrix.
</pre>

#### Constraints:
* `2 <= k <= 400`
* <code>1 <= rowConditions.length, colConditions.length <= 10<sup>4</sup></code>
* `rowConditions[i].length == colConditions[i].length == 2`
* <code>1 <= above<sub>i</sub>, below<sub>i</sub>, left<sub>i</sub>, right<sub>i</sub> <= k</code>
* <code>above<sub>i</sub> != below<sub>i</sub></code>
* <code>left<sub>i</sub> != right<sub>i</sub></code>

## Solutions (Rust)

### 1. Solution
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
