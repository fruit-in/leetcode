# 661. Image Smoother
Given a 2D integer matrix M representing the gray scale of an image, you need to design a smoother to make the gray scale of each cell becomes the average gray scale (rounding down) of all the 8 surrounding cells and itself. If a cell has less than 8 surrounding cells, then use as many as you can.

#### Example 1:
<pre>
<strong>Input:</strong>
[[1,1,1],
 [1,0,1],
 [1,1,1]]
<strong>Output:</strong>
[[0, 0, 0],
 [0, 0, 0],
 [0, 0, 0]]
<strong>Explanation:</strong>
For the point (0,0), (0,2), (2,0), (2,2): floor(3/4) = floor(0.75) = 0
For the point (0,1), (1,0), (1,2), (2,1): floor(5/6) = floor(0.83333333) = 0
For the point (1,1): floor(8/9) = floor(0.88888889) = 0
</pre>

#### Note:
1. The value in the given matrix is in the range of [0, 255].
2. The length and width of the given matrix are in the range of [1, 150].

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn image_smoother(m: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ret = vec![vec![0; m[0].len()]; m.len()];

        for i in 0..m.len() {
            for j in 0..m[0].len() {
                let mut cnt = 0;

                for k in i.saturating_sub(1)..=(i + 1).min(m.len() - 1) {
                    for l in j.saturating_sub(1)..=(j + 1).min(m[0].len() - 1) {
                        ret[i][j] += m[k][l];
                        cnt += 1;
                    }
                }

                ret[i][j] /= cnt;
            }
        }

        ret
    }
}
```
