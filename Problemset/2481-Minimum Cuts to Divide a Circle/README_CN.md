# 2481. 分割圆的最少切割次数
圆内一个 **有效切割** ，符合以下二者之一：

* 该切割是两个端点在圆上的线段，且该线段经过圆心。
* 该切割是一端在圆心另一端在圆上的线段。

一些有效和无效的切割如下图所示。

![](https://assets.leetcode.com/uploads/2022/10/29/alldrawio.png)

给你一个整数 `n` ，请你返回将圆切割成相等的 `n` 等分的 **最少** 切割次数。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/10/24/11drawio.png)
<pre>
<strong>输入:</strong> n = 4
<strong>输出:</strong> 2
<strong>解释:</strong>
上图展示了切割圆 2 次，得到四等分。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/10/24/22drawio.png)
<pre>
<strong>输入:</strong> n = 3
<strong>输出:</strong> 3
<strong>解释:</strong>
最少需要切割 3 次，将圆切成三等分。
少于 3 次切割无法将圆切成大小相等面积相同的 3 等分。
同时可以观察到，第一次切割无法将圆切割开。
</pre>

#### 提示:
* `1 <= n <= 100`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        if n == 1 || n % 2 == 0 {
            n / 2
        } else {
            n
        }
    }
}
```
