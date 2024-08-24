# 1992. Find All Groups of Farmland
You are given a **0-indexed** `m x n` binary matrix `land` where a `0` represents a hectare of forested land and a `1` represents a hectare of farmland.

To keep the land organized, there are designated rectangular areas of hectares that consist **entirely** of farmland. These rectangular areas are called **groups**. No two groups are adjacent, meaning farmland in one group is not four-directionally adjacent to another farmland in a different group.

`land` can be represented by a coordinate system where the top left corner of `land` is `(0, 0)` and the bottom right corner of `land` is `(m-1, n-1)`. Find the coordinates of the top left and bottom right corner of each **group** of farmland. A **group** of farmland with a top left corner at <code>(r<sub>1</sub>, c<sub>1</sub>)</code> and a bottom right corner at <code>(r<sub>2</sub>, c<sub>2</sub>)</code> is represented by the 4-length array <code>[r<sub>1</sub>, c<sub>1</sub>, r<sub>2</sub>, c<sub>2</sub>]</code>.

Return *a 2D array containing the 4-length arrays described above for each **group** of farmland in* `land`. *If there are no groups of farmland, return an empty array. You may return the answer in **any order***.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/07/27/screenshot-2021-07-27-at-12-23-15-copy-of-diagram-drawio-diagrams-net.png)
<pre>
<strong>Input:</strong> land = [[1,0,0],[0,1,1],[0,1,1]]
<strong>Output:</strong> [[0,0,0,0],[1,1,2,2]]
<strong>Explanation:</strong>
The first group has a top left corner at land[0][0] and a bottom right corner at land[0][0].
The second group has a top left corner at land[1][1] and a bottom right corner at land[2][2].
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/07/27/screenshot-2021-07-27-at-12-30-26-copy-of-diagram-drawio-diagrams-net.png)
<pre>
<strong>Input:</strong> land = [[1,1],[1,1]]
<strong>Output:</strong> [[0,0,1,1]]
<strong>Explanation:</strong>
The first group has a top left corner at land[0][0] and a bottom right corner at land[1][1].
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/07/27/screenshot-2021-07-27-at-12-32-24-copy-of-diagram-drawio-diagrams-net.png)
<pre>
<strong>Input:</strong> land = [[0]]
<strong>Output:</strong> []
<strong>Explanation:</strong>
There are no groups of farmland.
</pre>

#### Constraints:
* `m == land.length`
* `n == land[i].length`
* `1 <= m, n <= 300`
* `land` consists of only `0`'s and `1`'s.
* Groups of farmland are **rectangular** in shape.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = land.len();
        let n = land[0].len();
        let mut ret = vec![];

        for r1 in 0..m {
            for c1 in 0..n {
                if land[r1][c1] == 0
                    || (r1 > 0 && land[r1 - 1][c1] == 1)
                    || (c1 > 0 && land[r1][c1 - 1] == 1)
                {
                    continue;
                }

                let mut group = vec![r1 as i32, c1 as i32, r1 as i32, c1 as i32];

                for r2 in r1..m {
                    if land[r2][c1] == 1 {
                        group[2] = r2 as i32;
                    } else {
                        break;
                    }
                }
                for c2 in c1..n {
                    if land[group[2] as usize][c2] == 1 {
                        group[3] = c2 as i32;
                    } else {
                        break;
                    }
                }

                ret.push(group);
            }
        }

        ret
    }
}
```
