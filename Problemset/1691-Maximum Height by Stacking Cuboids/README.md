# 1691. Maximum Height by Stacking Cuboids
Given `n` `cuboids` where the dimensions of the <code>i<sup>th</sup></code> cuboid is <code>cuboids[i] = [width<sub>i</sub>, length<sub>i</sub>, height<sub>i</sub>]</code> (**0-indexed**). Choose a **subset** of `cuboids` and place them on each other.

You can place cuboid `i` on cuboid `j` if <code>width<sub>i</sub> <= width<sub>j</sub></code> and <code>length<sub>i</sub> <= length<sub>j</sub></code> and <code>height<sub>i</sub> <= height<sub>j</sub></code>. You can rearrange any cuboid's dimensions by rotating it to put it on another cuboid.

Return *the **maximum height** of the stacked* `cuboids`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/10/21/image.jpg)
<pre>
<strong>Input:</strong> cuboids = [[50,45,20],[95,37,53],[45,23,12]]
<strong>Output:</strong> 190
<strong>Explanation:</strong>
Cuboid 1 is placed on the bottom with the 53x37 side facing down with height 95.
Cuboid 0 is placed next with the 45x20 side facing down with height 50.
Cuboid 2 is placed next with the 23x12 side facing down with height 45.
The total height is 95 + 50 + 45 = 190.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> cuboids = [[38,25,45],[76,35,3]]
<strong>Output:</strong> 76
<strong>Explanation:</strong>
You can't place any of the cuboids on the other.
We choose cuboid 1 and rotate it so that the 35x3 side is facing down and its height is 76.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> cuboids = [[7,11,17],[7,17,11],[11,7,17],[11,17,7],[17,7,11],[17,11,7]]
<strong>Output:</strong> 102
<strong>Explanation:</strong>
After rearranging the cuboids, you can see that all cuboids have the same dimension.
You can place the 11x7 side down on all cuboids so their heights are 17.
The maximum height of stacked cuboids is 6 * 17 = 102.
</pre>

#### Constraints:
* `n == cuboids.length`
* `1 <= n <= 100`
* <code>1 <= width<sub>i</sub>, length<sub>i</sub>, height<sub>i</sub> <= 100</code>

## Solutions (Rust)

### 1. Solution
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
