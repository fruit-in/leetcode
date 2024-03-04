# 391. 完美矩形
给你一个数组 `rectangles` ，其中 <code>rectangles[i] = [x<sub>i</sub>, y<sub>i</sub>, a<sub>i</sub>, b<sub>i</sub>]</code> 表示一个坐标轴平行的矩形。这个矩形的左下顶点是 <code>(x<sub>i</sub>, y<sub>i</sub>)</code> ，右上顶点是 <code>(a<sub>i</sub>, b<sub>i</sub>)</code> 。

如果所有矩形一起精确覆盖了某个矩形区域，则返回 `true` ；否则，返回 `false` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/03/27/perectrec1-plane.jpg)
<pre>
<strong>输入:</strong> rectangles = [[1,1,3,3],[3,1,4,2],[3,2,4,4],[1,3,2,4],[2,3,3,4]]
<strong>输出:</strong> true
<strong>解释:</strong> 5 个矩形一起可以精确地覆盖一个矩形区域。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/03/27/perfectrec2-plane.jpg)
<pre>
<strong>输入:</strong> rectangles = [[1,1,2,3],[1,3,2,4],[3,1,4,2],[3,2,4,4]]
<strong>输出:</strong> false
<strong>解释:</strong> 两个矩形之间有间隔，无法覆盖成一个矩形。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/03/27/perfecrrec4-plane.jpg)
<pre>
<strong>输入:</strong> rectangles = [[1,1,3,3],[3,1,4,2],[1,3,2,4],[2,2,4,4]]
<strong>输出:</strong> false
<strong>解释:</strong> 因为中间有相交区域，虽然形成了矩形，但不是精确覆盖。
</pre>

#### 提示:
* <code>1 <= rectangles.length <= 2 * 10<sup>4</sup></code>
* `rectangles[i].length == 4`
* <code>-10<sup>5</sup> <= x<sub>i</sub>, y<sub>i</sub>, a<sub>i</sub>, b<sub>i</sub> <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let (mut minx, mut miny, mut maxa, mut maxb) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
        let mut area = 0;
        let mut count = HashMap::new();

        for rect in &rectangles {
            let (x, y, a, b) = (rect[0], rect[1], rect[2], rect[3]);
            minx = minx.min(x);
            miny = miny.min(y);
            maxa = maxa.max(a);
            maxb = maxb.max(b);
            area += (a - x) as i64 * (b - y) as i64;
            *count.entry((x, y)).or_insert(0) += 1;
            *count.entry((x, b)).or_insert(0) += 1;
            *count.entry((a, y)).or_insert(0) += 1;
            *count.entry((a, b)).or_insert(0) += 1;
        }

        if (maxa - minx) as i64 * (maxb - miny) as i64 != area {
            return false;
        }

        for (&(x, y), &c) in count.iter() {
            if (x == minx || x == maxa) && (y == miny || y == maxb) && c != 1 {
                return false;
            } else if ((x != minx && x != maxa) || (y != miny && y != maxb)) && c != 2 && c != 4 {
                return false;
            }
        }

        true
    }
}
```
