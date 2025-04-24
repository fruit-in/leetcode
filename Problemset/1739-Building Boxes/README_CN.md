# 1739. 放置盒子
有一个立方体房间，其长度、宽度和高度都等于 `n` 个单位。请你在房间里放置 `n` 个盒子，每个盒子都是一个单位边长的立方体。放置规则如下：
* 你可以把盒子放在地板上的任何地方。
* 如果盒子 `x` 需要放置在盒子 `y` 的顶部，那么盒子 `y` 竖直的四个侧面都 **必须** 与另一个盒子或墙相邻。

给你一个整数 `n` ，返回接触地面的盒子的 **最少** 可能数量。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/01/04/3-boxes.png)
<pre>
<strong>输入:</strong> n = 3
<strong>输出:</strong> 3
<strong>解释:</strong> 上图是 3 个盒子的摆放位置。
这些盒子放在房间的一角，对应左侧位置。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/01/04/4-boxes.png)
<pre>
<strong>输入:</strong> n = 4
<strong>输出:</strong> 3
<strong>解释:</strong> 上图是 3 个盒子的摆放位置。
这些盒子放在房间的一角，对应左侧位置。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/01/04/10-boxes.png)
<pre>
<strong>输入:</strong> n = 10
<strong>输出:</strong> 6
<strong>解释:</strong> 上图是 10 个盒子的摆放位置。
这些盒子放在房间的一角，对应后方位置。
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_boxes(n: i32) -> i32 {
        let n = n as i64;
        let mut lo = 1;
        let mut hi = n;

        while lo < hi {
            let mid = (lo + hi) / 2;
            let mut boxes = 0;
            let mut x = (mid as f64 * 2.).sqrt() as i64;
            let mut y = mid - x * (x + 1) / 2;

            if y < 0 {
                y += x;
                x -= 1;
            }

            while x > 0 {
                boxes += x * (x + 1) / 2 + y;
                x -= 1;
                y -= 1;
                y = y.max(0);
            }

            if boxes < n {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        hi as i32
    }
}
```
