# 2271. Maximum White Tiles Covered by a Carpet
You are given a 2D integer array `tiles` where <code>tiles[i] = [l<sub>i</sub>, r<sub>i</sub>]</code> represents that every tile `j` in the range <code>l<sub>i</sub> <= j <= r<sub>i</sub></code> is colored white.

You are also given an integer `carpetLen`, the length of a single carpet that can be placed **anywhere**.

Return *the **maximum** number of white tiles that can be covered by the carpet*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/03/25/example1drawio3.png)
<pre>
<strong>Input:</strong> tiles = [[1,5],[10,11],[12,18],[20,25],[30,32]], carpetLen = 10
<strong>Output:</strong> 9
<strong>Explanation:</strong> Place the carpet starting on tile 10.
It covers 9 white tiles, so we return 9.
Note that there may be other places where the carpet covers 9 white tiles.
It can be shown that the carpet cannot cover more than 9 white tiles.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/03/24/example2drawio.png)
<pre>
<strong>Input:</strong> tiles = [[10,11],[1,1]], carpetLen = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> Place the carpet starting on tile 10.
It covers 2 white tiles, so we return 2.
</pre>

#### Constraints:
* <code>1 <= tiles.length <= 5 * 10<sup>4</sup></code>
* `tiles[i].length == 2`
* <code>1 <= l<sub>i</sub> <= r<sub>i</sub> <= 10<sup>9</sup></code>
* <code>1 <= carpetLen <= 10<sup>9</sup></code>
* The `tiles` are **non-overlapping**.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn maximum_white_tiles(tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32 {
        let mut tiles = tiles;
        let mut prefix_sum = vec![0; tiles.len() + 1];
        let mut ret = 0;

        tiles.sort_unstable();

        for i in 1..prefix_sum.len() {
            prefix_sum[i] = prefix_sum[i - 1] + tiles[i - 1][1] - tiles[i - 1][0] + 1;
        }

        for i in 0..tiles.len() {
            let j = tiles
                .binary_search(&vec![tiles[i][0] + carpet_len, i32::MAX])
                .unwrap_err();
            if tiles[j - 1][1] < tiles[i][0] + carpet_len {
                ret = ret.max(prefix_sum[j] - prefix_sum[i]);
            } else {
                ret = ret.max(
                    prefix_sum[j] - prefix_sum[i] - tiles[j - 1][1] - tiles[i][0] - carpet_len,
                );
            }
            let j = tiles
                .binary_search(&vec![tiles[i][1] - carpet_len, i32::MAX])
                .unwrap_err();
            if j == 0 || tiles[j - 1][1] <= tiles[i][1] - carpet_len {
                ret = ret.max(prefix_sum[i + 1] - prefix_sum[j]);
            } else {
                ret = ret.max(
                    prefix_sum[i + 1] - prefix_sum[j] + tiles[j - 1][1] - tiles[i][1] + carpet_len,
                );
            }
        }

        ret
    }
}
```
