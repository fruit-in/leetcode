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
