# 780. Reaching Points
Given four integers `sx`, `sy`, `tx`, and `ty`, return `true` *if it is possible to convert the point* `(sx, sy)` *to the point* `(tx, ty)` *through some operations, or* `false` *otherwise*.

The allowed operation on some point `(x, y)` is to convert it to either `(x, x + y)` or `(x + y, y)`.

#### Example 1:
<pre>
<strong>Input:</strong> sx = 1, sy = 1, tx = 3, ty = 5
<strong>Output:</strong> true
<strong>Explanation:</strong>
One series of moves that transforms the starting point to the target is:
(1, 1) -> (1, 2)
(1, 2) -> (3, 2)
(3, 2) -> (3, 5)
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> sx = 1, sy = 1, tx = 2, ty = 2
<strong>Output:</strong> false
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> sx = 1, sy = 1, tx = 1, ty = 1
<strong>Output:</strong> true
</pre>

#### Constraints:
* <code>1 <= sx, sy, tx, ty <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        let mut tx = tx;
        let mut ty = ty;

        while tx > sx && ty > sy {
            if tx < ty {
                ty -= tx;
            } else {
                tx -= ty;
            }
        }

        if tx < sx || ty < sy {
            false
        } else if tx == sx && ty == sy {
            true
        } else if tx == sx {
            ty > tx && (ty - sy) % sx == 0
        } else {
            tx > ty && (tx - sx) % sy == 0
        }
    }
}
```
