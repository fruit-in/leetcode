# 780. 到达终点
给定四个整数 `sx` , `sy` ，`tx` 和 `ty`，如果通过一系列的**转换**可以从起点 `(sx, sy)` 到达终点 `(tx, ty)`，则返回 `true`，否则返回 `false`。

从点 `(x, y)` 可以**转换**到 `(x, x+y)`  或者 `(x+y, y)`。

#### 示例 1:
<pre>
<strong>输入:</strong> sx = 1, sy = 1, tx = 3, ty = 5
<strong>输出:</strong> true
<strong>解释:</strong>
可以通过以下一系列转换从起点转换到终点：
(1, 1) -> (1, 2)
(1, 2) -> (3, 2)
(3, 2) -> (3, 5)
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> sx = 1, sy = 1, tx = 2, ty = 2
<strong>输出:</strong> false
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> sx = 1, sy = 1, tx = 1, ty = 1
<strong>输出:</strong> true
</pre>

#### 提示:
* <code>1 <= sx, sy, tx, ty <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
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
