# 1691. 堆叠长方体的最大高度
给你 `n` 个长方体 `cuboids` ，其中第 `i` 个长方体的长宽高表示为 <code>cuboids[i] = [width<sub>i</sub>, length<sub>i</sub>, height<sub>i</sub>]</code>（**下标从 0 开始**）。请你从 `cuboids` 选出一个 **子集** ，并将它们堆叠起来。

如果 <code>width<sub>i</sub> <= width<sub>j</sub></code> 且 <code>length<sub>i</sub> <= length<sub>j</sub></code> 且 <code>height<sub>i</sub> <= height<sub>j</sub></code> ，你就可以将长方体 `i` 堆叠在长方体 `j` 上。你可以通过旋转把长方体的长宽高重新排列，以将它放在另一个长方体上。

返回 **堆叠长方体** `cuboids` 可以得到的 **最大高度** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2019/10/21/image.jpg)
<pre>
<strong>输入:</strong> cuboids = [[50,45,20],[95,37,53],[45,23,12]]
<strong>输出:</strong> 190
<strong>解释:</strong>
第 1 个长方体放在底部，53x37 的一面朝下，高度为 95 。
第 0 个长方体放在中间，45x20 的一面朝下，高度为 50 。
第 2 个长方体放在上面，23x12 的一面朝下，高度为 45 。
总高度是 95 + 50 + 45 = 190 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> cuboids = [[38,25,45],[76,35,3]]
<strong>输出:</strong> 76
<strong>解释:</strong>
无法将任何长方体放在另一个上面。
选择第 1 个长方体然后旋转它，使 35x3 的一面朝下，其高度为 76 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> cuboids = [[7,11,17],[7,17,11],[11,7,17],[11,17,7],[17,7,11],[17,11,7]]
<strong>输出:</strong> 102
<strong>解释:</strong>
重新排列长方体后，可以看到所有长方体的尺寸都相同。
你可以把 11x7 的一面朝下，这样它们的高度就是 17 。
堆叠长方体的最大高度为 6 * 17 = 102 。
</pre>

#### 提示:
* `n == cuboids.length`
* `1 <= n <= 100`
* <code>1 <= width<sub>i</sub>, length<sub>i</sub>, height<sub>i</sub> <= 100</code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_height(mut cuboids: Vec<Vec<i32>>) -> i32 {
        let mut max_width = 0;
        let mut max_length = 0;

        for cuboid in &mut cuboids {
            cuboid.sort_unstable();
            max_width = max_width.max(cuboid[0]);
            max_length = max_length.max(cuboid[1]);
        }

        cuboids.sort_unstable_by_key(|cuboid| (cuboid[2], cuboid[0], cuboid[1]));

        let mut dp = vec![vec![0; max_length as usize + 1]; max_width as usize + 1];

        for cuboid in &cuboids {
            let (width, length, height) = (cuboid[0] as usize, cuboid[1] as usize, cuboid[2]);

            for i in (0..=width).rev() {
                for j in (0..=length).rev() {
                    dp[width][length] = dp[width][length].max(dp[i][j] + height);
                }
            }
        }

        *dp.iter().flatten().max().unwrap()
    }
}
```
