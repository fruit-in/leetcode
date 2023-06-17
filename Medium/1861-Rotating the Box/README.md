# 1861. Rotating the Box
You are given an `m x n` matrix of characters `box` representing a side-view of a box. Each cell of the box is one of the following:

* A stone `'#'`
* A stationary obstacle `'*'`
* Empty `'.'`

The box is rotated **90 degrees clockwise**, causing some of the stones to fall due to gravity. Each stone falls down until it lands on an obstacle, another stone, or the bottom of the box. Gravity **does not** affect the obstacles' positions, and the inertia from the box's rotation **does not** affect the stones' horizontal positions.

It is **guaranteed** that each stone in `box` rests on an obstacle, another stone, or the bottom of the box.

Return *an* `n x m` *matrix representing the box after the rotation described above*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/04/08/rotatingtheboxleetcodewithstones.png)
<pre>
<strong>Input:</strong> box = [["#",".","#"]]
<strong>Output:</strong> [["."],
         ["#"],
         ["#"]]
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/04/08/rotatingtheboxleetcode2withstones.png)
<pre>
<strong>Input:</strong> box = [["#",".","*","."],
              ["#","#","*","."]]
<strong>Output:</strong> [["#","."],
         ["#","#"],
         ["*","*"],
         [".","."]]
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/04/08/rotatingtheboxleetcode3withstone.png)
<pre>
<strong>Input:</strong> box = [["#","#","*",".","*","."],
              ["#","#","#","*",".","."],
              ["#","#","#",".","#","."]]
<strong>Output:</strong> [[".","#","#"],
         [".","#","#"],
         ["#","#","*"],
         ["#","*","."],
         ["#",".","*"],
         ["#",".","."]]
</pre>

#### Constraints:
* `m == box.length`
* `n == box[i].length`
* `1 <= m, n <= 500`
* `box[i][j]` is either `'#'`, `'*'`, or `'.'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn rotate_the_box(bbox: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let m = bbox.len();
        let n = bbox[0].len();
        let mut count = 0;
        let mut ret = vec![vec!['.'; m]; n];

        for i in 0..m {
            for j in 0..n {
                if bbox[i][j] == '#' {
                    count += 1;
                } else if bbox[i][j] == '*' {
                    ret[j][m - i - 1] = '*';

                    for k in 0..count {
                        ret[j - k - 1][m - i - 1] = '#';
                    }
                    count = 0;
                }
            }

            for k in 0..count {
                ret[n - k - 1][m - i - 1] = '#';
            }
            count = 0;
        }

        ret
    }
}
```
