# 1030. Matrix Cells in Distance Order
We are given a matrix with ```R``` rows and ```C``` columns has cells with integer coordinates ```(r, c)```, where ```0 <= r < R``` and ```0 <= c < C```.

Additionally, we are given a cell in that matrix with coordinates ```(r0, c0)```.

Return the coordinates of all cells in the matrix, sorted by their distance from ```(r0, c0)``` from smallest distance to largest distance.  Here, the distance between two cells ```(r1, c1)``` and ```(r2, c2)``` is the Manhattan distance, ```|r1 - r2| + |c1 - c2|```.  (You may return the answer in any order that satisfies this condition.)

#### Example 1:
<pre>
<strong>Input:</strong> R = 1, C = 2, r0 = 0, c0 = 0
<strong>Output:</strong> [[0,0],[0,1]]
<strong>Explanation:</strong> The distances from (r0, c0) to other cells are: [0,1]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> R = 2, C = 2, r0 = 0, c0 = 1
<strong>Output:</strong> [[0,1],[0,0],[1,1],[1,0]]
<strong>Explanation:</strong> The distances from (r0, c0) to other cells are: [0,1,1,2]
The answer [[0,1],[1,1],[0,0],[1,0]] would also be accepted as correct.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> R = 2, C = 3, r0 = 1, c0 = 2
<strong>Output:</strong> [[1,2],[0,2],[1,1],[0,1],[1,0],[0,0]]
<strong>Explanation:</strong> The distances from (r0, c0) to other cells are: [0,1,1,2,2,3]
There are other answers that would also be accepted as correct, such as [[1,2],[1,1],[0,2],[1,0],[0,1],[0,0]].
</pre>

#### Note:
1. ```1 <= R <= 100```
2. ```1 <= C <= 100```
3. ```0 <= r0 < R```
4. ```0 <= c0 < C```

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut cells = Vec::new();
        for i in 0..r {
            for j in 0..c {
                cells.push(vec![i, j]);
            }
        }

        cells.sort_unstable_by_key(|v| (v[0] - r0).abs() + (v[1] - c0).abs());

        cells
    }
}
```

### 2. Construct by Distance
```Rust
impl Solution {
    pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut max_dis = r0.max(r - 1 - r0) + c0.max(c - 1 - c0);
        let mut cells = Vec::new();

        for dis in 0..=max_dis {
            for r_diff in (-dis).max(-r0)..=dis.min(r - 1 - r0) {
                let c_diff = dis - r_diff.abs();

                if c_diff == 0 {
                    cells.push(vec![r0 + r_diff, c0]);
                } else {
                    if c0 - c_diff > -1 {
                        cells.push(vec![r0 + r_diff, c0 - c_diff]);
                    }
                    if c0 + c_diff < c {
                        cells.push(vec![r0 + r_diff, c0 + c_diff]);
                    }
                }
            }
        }

        cells
    }
}
```
