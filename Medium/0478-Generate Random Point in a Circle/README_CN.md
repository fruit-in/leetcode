# 478. 在圆内随机生成点
给定圆的半径和圆心的 x、y 坐标，写一个在圆中产生均匀随机点的函数 `randPoint` 。

说明:
1. 输入值和输出值都将是[浮点数](https://baike.baidu.com/item/%E6%B5%AE%E7%82%B9%E6%95%B0/6162520)。
2. 圆的半径和圆心的 x、y 坐标将作为参数传递给类的构造函数。
3. 圆周上的点也认为是在圆中。
4. `randPoint` 返回一个包含随机点的x坐标和y坐标的大小为2的数组。

#### 示例 1:
<pre>
<strong>输入:</strong>
["Solution","randPoint","randPoint","randPoint"]
[[1,0,0],[],[],[]]
<strong>输出:</strong> [null,[-0.72939,-0.65505],[-0.78502,-0.28626],[-0.83119,-0.19803]]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong>
["Solution","randPoint","randPoint","randPoint"]
[[10,5,-7.5],[],[],[]]
<strong>输出:</strong> [null,[11.52438,-8.33273],[2.46992,-16.21705],[11.13430,-12.42337]]
</pre>

#### 输入语法说明:
输入是两个列表：调用成员函数名和调用的参数。`Solution` 的构造函数有三个参数，圆的半径、圆心的 x 坐标、圆心的 y 坐标。`randPoint` 没有参数。输入参数是一个列表，即使参数为空，也会输入一个 [] 空列表。

## 题解 (Rust)

### 1. 题解
```Rust
use rand::{thread_rng, Rng};

struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            radius,
            x_center,
            y_center,
        }
    }

    fn rand_point(&self) -> Vec<f64> {
        let mut rng = thread_rng();
        let delta_x = rng.gen_range(-self.radius, self.radius);
        let delta_y = rng.gen_range(-self.radius, self.radius);

        if delta_x.powi(2) + delta_y.powi(2) <= self.radius.powi(2) {
            vec![self.x_center + delta_x, self.y_center + delta_y]
        } else {
            self.rand_point()
        }
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(radius, x_center, y_center);
 * let ret_1: Vec<f64> = obj.rand_point();
 */
```
