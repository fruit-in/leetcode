# 2271. 毯子覆盖的最多白色砖块数
给你一个二维整数数组 `tiles` ，其中 <code>tiles[i] = [l<sub>i</sub>, r<sub>i</sub>]</code> ，表示所有在 <code>l<sub>i</sub> <= j <= r<sub>i</sub></code> 之间的每个瓷砖位置 `j` 都被涂成了白色。

同时给你一个整数 `carpetLen` ，表示可以放在 **任何位置** 的一块毯子的长度。

请你返回使用这块毯子，**最多** 可以盖住多少块瓷砖。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/03/25/example1drawio3.png)
<pre>
<strong>输入:</strong> tiles = [[1,5],[10,11],[12,18],[20,25],[30,32]], carpetLen = 10
<strong>输出:</strong> 9
<strong>解释:</strong> 将毯子从瓷砖 10 开始放置。
总共覆盖 9 块瓷砖，所以返回 9 。
注意可能有其他方案也可以覆盖 9 块瓷砖。
可以看出，瓷砖无法覆盖超过 9 块瓷砖。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/03/24/example2drawio.png)
<pre>
<strong>输入:</strong> tiles = [[10,11],[1,1]], carpetLen = 2
<strong>输出:</strong> 2
<strong>解释:</strong> 将毯子从瓷砖 10 开始放置。
总共覆盖 2 块瓷砖，所以我们返回 2 。
</pre>

#### 提示:
* <code>1 <= tiles.length <= 5 * 10<sup>4</sup></code>
* `tiles[i].length == 2`
* <code>1 <= l<sub>i</sub> <= r<sub>i</sub> <= 10<sup>9</sup></code>
* <code>1 <= carpetLen <= 10<sup>9</sup></code>
* `tiles` 互相 **不会重叠** 。

## 题解 (Rust)

### 1. 题解
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
