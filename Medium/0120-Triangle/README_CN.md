# 120. 三角形最小路径和
给定一个三角形，找出自顶向下的最小路径和。每一步只能移动到下一行中相邻的结点上。

例如，给定三角形：

<pre>
[
     [<strong>2</strong>],
    [<strong>3</strong>,4],
   [6,<strong>5</strong>,7],
  [4,<strong>1</strong>,8,3]
]
</pre>

自顶向下的最小路径和为 ```11```（即，**2** + **3** + **5** + **1** = 11）。

#### 说明:
如果你可以只使用 *O*(*n*) 的额外空间（*n* 为三角形的总行数）来解决这个问题，那么你的算法会很加分。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut triangle = triangle;

        for r in 1..triangle.len() {
            triangle[r][0] += triangle[r - 1][0];
            triangle[r][r] += triangle[r - 1][r - 1];
            for i in 1..(triangle[r].len() - 1) {
                triangle[r][i] += triangle[r - 1][i - 1].min(triangle[r - 1][i])
            }
        }

        *triangle.last().unwrap().iter().min().unwrap()
    }
}
```
