# 554. 砖墙
你的面前有一堵方形的、由多行砖块组成的砖墙。 这些砖块高度相同但是宽度不同。你现在要画一条**自顶向下**的、穿过**最少**砖块的垂线。

砖墙由行的列表表示。 每一行都是一个代表从左至右每块砖的宽度的整数列表。

如果你画的线只是从砖块的边缘经过，就不算穿过这块砖。你需要找出怎样画才能使这条线穿过的砖块数量最少，并且返回穿过的砖块数量。

**你不能沿着墙的两个垂直边缘之一画线，这样显然是没有穿过一块砖的。**

#### 示例:
<pre>
<strong>输入:</strong> [[1,2,2,1],
        [3,1,2],
        [1,3,2],
        [2,4],
        [3,1,2],
        [1,3,1,1]]
<strong>输出:</strong> 2
<strong>解释:</strong>
<img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/12/brick_wall.png">
</pre>

#### 提示:
1. 每一行砖块的宽度之和应该相等，并且不能超过 INT_MAX。
2. 每一行砖块的数量在 [1,10,000] 范围内， 墙的高度在 [1,10,000] 范围内， 总的砖块数量不超过 20,000。

## 题解 (Rust)

### 1. 哈希表
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let mut map = HashMap::new();

        for row in &wall {
            let mut sum = 0;
            for i in 0..(row.len() - 1) {
                sum += row[i];
                *map.entry(sum).or_insert(0) += 1;
            }
        }

        wall.len() as i32 - map.values().max().unwrap_or(&0)
    }
}
```
