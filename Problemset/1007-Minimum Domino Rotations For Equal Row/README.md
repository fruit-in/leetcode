# 1007. Minimum Domino Rotations For Equal Row
In a row of dominoes, `tops[i]` and `bottoms[i]` represent the top and bottom halves of the <code>i<sup>th</sup></code> domino. (A domino is a tile with two numbers from 1 to 6 - one on each half of the tile.)

We may rotate the <code>i<sup>th</sup></code> domino, so that `tops[i]` and `bottoms[i]` swap values.

Return the minimum number of rotations so that all the values in `tops` are the same, or all the values in `bottoms` are the same.

If it cannot be done, return `-1`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/05/14/domino.png)
<pre>
<strong>Input:</strong> tops = [2,1,2,4,2,2], bottoms = [5,2,6,2,3,2]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
The first figure represents the dominoes as given by tops and bottoms: before we do any rotations.
If we rotate the second and fourth dominoes, we can make every value in the top row equal to 2, as indicated by the second figure.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> tops = [3,5,1,2,3], bottoms = [3,6,3,3,4]
<strong>Output:</strong> -1
<strong>Explanation:</strong>
In this case, it is not possible to rotate the dominoes to make one row of values equal.
</pre>

#### Constraints:
* <code>2 <= tops.length <= 2 * 10<sup>4</sup></code>
* `bottoms.length == tops.length`
* `1 <= tops[i], bottoms[i] <= 6`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let mut rotate_top = [0, 1];
        let mut rotate_bottom = [0, 1];

        for i in 1..tops.len() {
            if rotate_top[0] >= 0 && tops[i] != tops[0] && bottoms[i] == tops[0] {
                rotate_top[0] += 1;
            } else if tops[i] != tops[0] {
                rotate_top[0] = -1;
            }
            if rotate_top[1] >= 0 && bottoms[i] != tops[0] && tops[i] == tops[0] {
                rotate_top[1] += 1;
            } else if bottoms[i] != tops[0] {
                rotate_top[1] = -1;
            }
            if rotate_bottom[0] >= 0 && bottoms[i] != bottoms[0] && tops[i] == bottoms[0] {
                rotate_bottom[0] += 1;
            } else if bottoms[i] != bottoms[0] {
                rotate_bottom[0] = -1;
            }
            if rotate_bottom[1] >= 0 && tops[i] != bottoms[0] && bottoms[i] == bottoms[0] {
                rotate_bottom[1] += 1;
            } else if tops[i] != bottoms[0] {
                rotate_bottom[1] = -1;
            }
        }

        *rotate_top
            .iter()
            .chain(rotate_bottom.iter())
            .filter(|&&x| x >= 0)
            .min()
            .unwrap_or(&-1)
    }
}
```
