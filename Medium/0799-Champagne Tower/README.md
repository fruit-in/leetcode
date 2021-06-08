# 799. Champagne Tower
We stack glasses in a pyramid, where the **first** row has `1` glass, the **second** row has `2` glasses, and so on until the 100<sup>th</sup> row.  Each glass holds one cup of champagne.

Then, some champagne is poured into the first glass at the top.  When the topmost glass is full, any excess liquid poured will fall equally to the glass immediately to the left and right of it.  When those glasses become full, any excess champagne will fall equally to the left and right of those glasses, and so on.  (A glass at the bottom row has its excess champagne fall on the floor.)

For example, after one cup of champagne is poured, the top most glass is full.  After two cups of champagne are poured, the two glasses on the second row are half full.  After three cups of champagne are poured, those two cups become full - there are 3 full glasses total now.  After four cups of champagne are poured, the third row has the middle glass half full, and the two outside glasses are a quarter full, as pictured below.

![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/03/09/tower.png)

Now after pouring some non-negative integer cups of champagne, return how full the <code>j<sup>th</sup></code> glass in the <code>i<sup>th</sup></code> row is (both `i` and `j` are 0-indexed.)

#### Example 1:
<pre>
<strong>Input:</strong> poured = 1, query_row = 1, query_glass = 1
<strong>Output:</strong> 0.00000
<strong>Explanation:</strong> We poured 1 cup of champange to the top glass of the tower (which is indexed as (0, 0)). There will be no excess liquid so all the glasses under the top glass will remain empty.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> poured = 2, query_row = 1, query_glass = 1
<strong>Output:</strong> 0.50000
<strong>Explanation:</strong> We poured 2 cups of champange to the top glass of the tower (which is indexed as (0, 0)). There is one cup of excess liquid. The glass indexed as (1, 0) and the glass indexed as (1, 1) will share the excess liquid equally, and each will get half cup of champange.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> poured = 100000009, query_row = 33, query_glass = 17
<strong>Output:</strong> 1.00000
</pre>

#### Constraints:
* <code>0 <= poured <= 10<sup>9</sup></code>
* `0 <= query_glass <= query_row < 100`

## Solutions (Rust)

### 1. Dynamic Programming
```Rust
impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut dp = vec![poured as f64];

        for r in 0..query_row as usize {
            dp.push((dp[r] - 1.0).max(0.0) / 2.0);
            for i in (1..=r).rev() {
                dp[i] = (dp[i] - 1.0).max(0.0) / 2.0 + (dp[i - 1] - 1.0).max(0.0) / 2.0;
            }
            dp[0] = (dp[0] - 1.0).max(0.0) / 2.0;
        }

        dp[query_glass as usize].min(1.0)
    }
}
```
