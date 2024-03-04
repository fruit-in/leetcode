# 391. Perfect Rectangle
Given an array `rectangles` where <code>rectangles[i] = [x<sub>i</sub>, y<sub>i</sub>, a<sub>i</sub>, b<sub>i</sub>]</code> represents an axis-aligned rectangle. The bottom-left point of the rectangle is <code>(x<sub>i</sub>, y<sub>i</sub>)</code> and the top-right point of it is <code>(a<sub>i</sub>, b<sub>i</sub>)</code>.

Return `true` *if all the rectangles together form an exact cover of a rectangular region*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/03/27/perectrec1-plane.jpg)
<pre>
<strong>Input:</strong> rectangles = [[1,1,3,3],[3,1,4,2],[3,2,4,4],[1,3,2,4],[2,3,3,4]]
<strong>Output:</strong> true
<strong>Explanation:</strong> All 5 rectangles together form an exact cover of a rectangular region.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/03/27/perfectrec2-plane.jpg)
<pre>
<strong>Input:</strong> rectangles = [[1,1,2,3],[1,3,2,4],[3,1,4,2],[3,2,4,4]]
<strong>Output:</strong> false
<strong>Explanation:</strong> Because there is a gap between the two rectangular regions.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/03/27/perfecrrec4-plane.jpg)
<pre>
<strong>Input:</strong> rectangles = [[1,1,3,3],[3,1,4,2],[1,3,2,4],[2,2,4,4]]
<strong>Output:</strong> false
<strong>Explanation:</strong> Because two of the rectangles overlap with each other.
</pre>

#### Constraints:
* <code>1 <= rectangles.length <= 2 * 10<sup>4</sup></code>
* `rectangles[i].length == 4`
* <code>-10<sup>5</sup> <= x<sub>i</sub>, y<sub>i</sub>, a<sub>i</sub>, b<sub>i</sub> <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
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
