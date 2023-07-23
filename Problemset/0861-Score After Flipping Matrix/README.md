# 861. Score After Flipping Matrix
We have a two dimensional matrix `A` where each value is `0` or `1`.

A move consists of choosing any row or column, and toggling each value in that row or column: changing all `0`s to `1`s, and all `1`s to `0`s.

After making any number of moves, every row of this matrix is interpreted as a binary number, and the score of the matrix is the sum of these numbers.

Return the highest possible score.

#### Example 1:
<pre>
<b>Input:</b> [[0,0,1,1],[1,0,1,0],[1,1,0,0]]
<b>Output:</b> 39
<b>Explanation:</b>
Toggled to [[1,1,1,1],[1,0,0,1],[1,1,1,1]].
0b1111 + 0b1001 + 0b1111 = 15 + 9 + 15 = 39
</pre>

#### Note:
1. `1 <= A.length <= 20`
2. `1 <= A[0].length <= 20`
3. `A[i][j]` is `0` or `1`.

## Solutions (Rust)

### 1. Greedy
```Rust
impl Solution {
    pub fn matrix_score(a: Vec<Vec<i32>>) -> i32 {
        let m = a.len();
        let n = a[0].len();
        let mut ret = 0;

        for c in 0..n {
            let mut zeros = 0;

            for r in 0..m {
                zeros += a[r][0] ^ a[r][c];
            }

            ret += zeros.max(m as i32 - zeros) * (1 << (a[0].len() - 1 - c));
        }

        ret
    }
}
```
