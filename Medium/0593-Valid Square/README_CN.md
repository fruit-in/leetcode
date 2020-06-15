# 593. 有效的正方形
给定二维空间中四点的坐标，返回四点是否可以构造一个正方形。

一个点的坐标（x，y）由一个有两个整数的整数数组表示。

#### 示例:
<pre>
<strong>输入:</strong> p1 = [0,0], p2 = [1,1], p3 = [1,0], p4 = [0,1]
<strong>输出:</strong> True
</pre>

#### 注意:
1. 所有输入整数都在 [-10000，10000] 范围内。
2. 一个有效的正方形有四个等长的正长和四个等角（90度角）。
3. 输入点没有顺序。

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let points = [p1, p2, p3, p4];
        let mut dis_2 = Vec::new();

        for i in 1..4 {
            for j in 0..i {
                let x = points[i][0] - points[j][0];
                let y = points[i][1] - points[j][1];
                dis_2.push(x * x + y * y);
            }
        }

        dis_2.sort_unstable();

        dis_2[0] != 0 && dis_2[0] == dis_2[3] && dis_2[4] == dis_2[5]
    }
}
```
