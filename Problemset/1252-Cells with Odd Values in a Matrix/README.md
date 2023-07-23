# 1252. Cells with Odd Values in a Matrix
Given ```n``` and ```m``` which are the dimensions of a matrix initialized by zeros and given an array ```indices``` where ```indices[i] = [ri, ci]```. For each pair of ```[ri, ci]``` you have to increment all cells in row ```ri``` and column ```ci``` by 1.

Return *the number of cells with odd values* in the matrix after applying the increment to all ```indices```.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/10/30/e1.png)
<pre>
<strong>Input:</strong> n = 2, m = 3, indices = [[0,1],[1,1]]
<strong>Output:</strong> 6
<strong>Explanation:</strong> Initial matrix = [[0,0,0],[0,0,0]].
After applying first increment it becomes [[1,2,1],[0,1,0]].
The final matrix will be [[1,3,1],[1,3,1]] which contains 6 odd numbers.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2019/10/30/e2.png)
<pre>
<strong>Input:</strong> n = 2, m = 2, indices = [[1,1],[0,0]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> Final matrix = [[2,2],[2,2]]. There is no odd number in the final matrix.
</pre>

#### Constraints:
* ```1 <= n <= 50```
* ```1 <= m <= 50```
* ```1 <= indices.length <= 100```
* ```0 <= indices[i][0] < n```
* ```0 <= indices[i][1] < m```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn odd_cells(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut matrix = vec![vec![false; m as usize]; n as usize];

        for index in indices {
            let ri = index[0] as usize;
            let ci = index[1] as usize;

            for c in 0..(m as usize) {
                matrix[ri][c] = !matrix[ri][c];
            }
            for r in 0..(n as usize) {
                matrix[r][ci] = !matrix[r][ci];
            }
        }

        matrix.iter().map(|r| r.iter().filter(|&&c| c).count() as i32).sum()
    }
}
```
