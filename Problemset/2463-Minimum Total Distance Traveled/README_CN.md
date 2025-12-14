# 2463. 最小移动总距离
X 轴上有一些机器人和工厂。给你一个整数数组 `robot` ，其中 `robot[i]` 是第 `i` 个机器人的位置。再给你一个二维整数数组 `factory` ，其中 <code>factory[j] = [position<sub>j</sub>, limit<sub>j</sub>]</code> ，表示第 `j` 个工厂的位置在 <code>position<sub>j</sub></code> ，且第 `j` 个工厂最多可以修理 <code>limit<sub>j</sub></code> 个机器人。

每个机器人所在的位置 **互不相同** 。每个工厂所在的位置也 **互不相同** 。注意一个机器人可能一开始跟一个工厂在 **相同的位置** 。

所有机器人一开始都是坏的，他们会沿着设定的方向一直移动。设定的方向要么是 X 轴的正方向，要么是 X 轴的负方向。当一个机器人经过一个没达到上限的工厂时，这个工厂会维修这个机器人，且机器人停止移动。

**任何时刻**，你都可以设置 **部分** 机器人的移动方向。你的目标是最小化所有机器人总的移动距离。

请你返回所有机器人移动的最小总距离。测试数据保证所有机器人都可以被维修。

#### 注意：
* 所有机器人移动速度相同。
* 如果两个机器人移动方向相同，它们永远不会碰撞。
* 如果两个机器人迎面相遇，它们也不会碰撞，它们彼此之间会擦肩而过。
* 如果一个机器人经过了一个已经达到上限的工厂，机器人会当作工厂不存在，继续移动。
* 机器人从位置 `x` 到位置 `y` 的移动距离为 `|y - x|` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/09/15/example1.jpg)
<pre>
<strong>输入:</strong> robot = [0,4,6], factory = [[2,2],[6,2]]
<strong>输出:</strong> 4
<strong>解释:</strong> 如上图所示：
- 第一个机器人从位置 0 沿着正方向移动，在第一个工厂处维修。
- 第二个机器人从位置 4 沿着负方向移动，在第一个工厂处维修。
- 第三个机器人在位置 6 被第二个工厂维修，它不需要移动。
第一个工厂的维修上限是 2 ，它维修了 2 个机器人。
第二个工厂的维修上限是 2 ，它维修了 1 个机器人。
总移动距离是 |2 - 0| + |2 - 4| + |6 - 6| = 4 。没有办法得到比 4 更少的总移动距离。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/09/15/example-2.jpg)
<pre>
<strong>输入:</strong> robot = [1,-1], factory = [[-2,1],[2,1]]
<strong>输出:</strong> 2
<strong>解释:</strong> 如上图所示：
- 第一个机器人从位置 1 沿着正方向移动，在第二个工厂处维修。
- 第二个机器人在位置 -1 沿着负方向移动，在第一个工厂处维修。
第一个工厂的维修上限是 1 ，它维修了 1 个机器人。
第二个工厂的维修上限是 1 ，它维修了 1 个机器人。
总移动距离是 |2 - 1| + |(-2) - (-1)| = 2 。没有办法得到比 2 更少的总移动距离。
</pre>

#### 提示:
* `1 <= robot.length, factory.length <= 100`
* `factory[j].length == 2`
* <code>-10<sup>9</sup> <= robot[i], position<sub>j</sub> <= 10<sup>9</sup></code>
* <code>0 <= limit<sub>j</sub> <= robot.length</code>
* 测试数据保证所有机器人都可以被维修。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
        robot.sort_unstable();
        factory.sort_unstable();

        let m = robot.len();
        let n = factory.len();
        let mut dp = vec![vec![i64::MAX; m + 1]; n + 1];
        dp[0][0] = 0;

        for i in 1..=n {
            for j in 0..=m {
                let mut distance = 0;
                dp[i][j] = dp[i - 1][j];

                for k in 1..=j.min(factory[i - 1][1] as usize) {
                    distance += (factory[i - 1][0] - robot[j - k]).abs() as i64;
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - k].saturating_add(distance));
                }
            }
        }

        dp[n][m]
    }
}
```
