# 2013. 检测正方形
给你一个在 X-Y 平面上的点构成的数据流。设计一个满足下述要求的算法：

* **添加** 一个在数据流中的新点到某个数据结构中。可以添加 **重复** 的点，并会视作不同的点进行处理。
* 给你一个查询点，请你从数据结构中选出三个点，使这三个点和查询点一同构成一个 **面积为正** 的 **轴对齐正方形** ，**统计** 满足该要求的方案数目。

**轴对齐正方形** 是一个正方形，除四条边长度相同外，还满足每条边都与 x-轴 或 y-轴 平行或垂直。

实现 `DetectSquares` 类：

* `DetectSquares()` 使用空数据结构初始化对象
* `void add(int[] point)` 向数据结构添加一个新的点 `point = [x, y]`
* `int count(int[] point)` 统计按上述方式与点 `point = [x, y]` 共同构造 **轴对齐正方形** 的方案数。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/09/01/image.png)
<pre>
<strong>输入:</strong>
["DetectSquares", "add", "add", "add", "count", "count", "add", "count"]
[[], [[3, 10]], [[11, 2]], [[3, 2]], [[11, 10]], [[14, 8]], [[11, 2]], [[11, 10]]]
<strong>输出:</strong>
[null, null, null, null, 1, 0, null, 2]
<strong>解释:</strong>
DetectSquares detectSquares = new DetectSquares();
detectSquares.add([3, 10]);
detectSquares.add([11, 2]);
detectSquares.add([3, 2]);
detectSquares.count([11, 10]); // 返回 1 。你可以选择：
                               //   - 第一个，第二个，和第三个点
detectSquares.count([14, 8]);  // 返回 0 。查询点无法与数据结构中的这些点构成正方形。
detectSquares.add([11, 2]);    // 允许添加重复的点。
detectSquares.count([11, 10]); // 返回 2 。你可以选择：
                               //   - 第一个，第二个，和第三个点
                               //   - 第一个，第三个，和第四个点
</pre>

#### 提示:
* `point.length == 2`
* `0 <= x, y <= 1000`
* 调用 `add` 和 `count` 的 **总次数** 最多为 `5000`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

struct DetectSquares {
    points: HashMap<(i32, i32), i32>,
    diagonals: HashMap<(i32, i32), Vec<(i32, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DetectSquares {
    fn new() -> Self {
        Self {
            points: HashMap::new(),
            diagonals: HashMap::new(),
        }
    }

    fn add(&mut self, point: Vec<i32>) {
        let (x, y) = (point[0], point[1]);

        *self.points.entry((x, y)).or_insert(0) += 1;
        self.diagonals
            .entry((1, y - x))
            .or_insert(vec![])
            .push((x, y));
        self.diagonals
            .entry((-1, y + x))
            .or_insert(vec![])
            .push((x, y));
    }

    fn count(&self, point: Vec<i32>) -> i32 {
        let (x0, y0) = (point[0], point[1]);
        let mut ret = 0;

        for &(x1, y1) in self.diagonals.get(&(1, y0 - x0)).unwrap_or(&vec![]) {
            if x1 == x0 {
                continue;
            }

            let count0 = *self.points.get(&(x0, y1)).unwrap_or(&0);
            let count1 = *self.points.get(&(x1, y0)).unwrap_or(&0);
            ret += count0 * count1;
        }
        for &(x1, y1) in self.diagonals.get(&(-1, y0 + x0)).unwrap_or(&vec![]) {
            if x1 == x0 {
                continue;
            }

            let count0 = *self.points.get(&(x0, y1)).unwrap_or(&0);
            let count1 = *self.points.get(&(x1, y0)).unwrap_or(&0);
            ret += count0 * count1;
        }

        ret
    }
}

/**
 * Your DetectSquares object will be instantiated and called as such:
 * let obj = DetectSquares::new();
 * obj.add(point);
 * let ret_2: i32 = obj.count(point);
 */
```
