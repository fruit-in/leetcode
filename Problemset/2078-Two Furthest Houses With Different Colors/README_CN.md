# 2078. 两栋颜色不同且距离最远的房子
街上有 `n` 栋房子整齐地排成一列，每栋房子都粉刷上了漂亮的颜色。给你一个下标从 **0** 开始且长度为 `n` 的整数数组 `colors` ，其中 `colors[i]` 表示第  `i` 栋房子的颜色。

返回 **两栋** 颜色 **不同** 房子之间的 **最大** 距离。

第 `i` 栋房子和第 `j` 栋房子之间的距离是 `abs(i - j)` ，其中 `abs(x)` 是 `x` 的绝对值。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/10/31/eg1.png)
<pre>
<strong>输入:</strong> colors = [1,1,1,6,1,1,1]
<strong>输出:</strong> 3
<strong>解释:</strong> 上图中，颜色 1 标识成蓝色，颜色 6 标识成红色。
两栋颜色不同且距离最远的房子是房子 0 和房子 3 。
房子 0 的颜色是颜色 1 ，房子 3 的颜色是颜色 6 。两栋房子之间的距离是 abs(0 - 3) = 3 。
注意，房子 3 和房子 6 也可以产生最佳答案。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/10/31/eg2.png)
<pre>
<strong>输入:</strong> colors = [1,8,3,8,3]
<strong>输出:</strong> 4
<strong>解释:</strong> 上图中，颜色 1 标识成蓝色，颜色 8 标识成黄色，颜色 3 标识成绿色。
两栋颜色不同且距离最远的房子是房子 0 和房子 4 。
房子 0 的颜色是颜色 1 ，房子 4 的颜色是颜色 3 。两栋房子之间的距离是 abs(0 - 4) = 4 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> colors = [0,1]
<strong>输出:</strong> 1
<strong>解释:</strong> 两栋颜色不同且距离最远的房子是房子 0 和房子 1 。
房子 0 的颜色是颜色 0 ，房子 1 的颜色是颜色 1 。两栋房子之间的距离是 abs(0 - 1) = 1 。
</pre>

#### 提示:
* `n == colors.length`
* `2 <= n <= 100`
* `0 <= colors[i] <= 100`
* 生成的测试数据满足 **至少** 存在 2 栋颜色不同的房子

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let n = colors.len();
        let mut ret = n - 1;

        while colors[0] == colors[ret] && colors[n - 1] == colors[n - 1 - ret] {
            ret -= 1;
        }

        ret as i32
    }
}
```
