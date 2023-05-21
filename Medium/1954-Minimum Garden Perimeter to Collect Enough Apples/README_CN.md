# 1954. 收集足够苹果的最小花园周长
给你一个用无限二维网格表示的花园，**每一个** 整数坐标处都有一棵苹果树。整数坐标 `(i, j)` 处的苹果树有 `|i| + |j|` 个苹果。

你将会买下正中心坐标是 `(0, 0)` 的一块 **正方形土地** ，且每条边都与两条坐标轴之一平行。

给你一个整数 `neededApples` ，请你返回土地的 **最小周长** ，使得 **至少** 有 `neededApples` 个苹果在土地 **里面或者边缘上**。

`|x|` 的值定义为：

* 如果 `x >= 0` ，那么值为 `x`
* 如果 `x < 0` ，那么值为 `-x`

#### 示例 1:
![](https://assets.leetcode.com/uploads/2019/08/30/1527_example_1_2.png)
<pre>
<strong>输入:</strong> neededApples = 1
<strong>输出:</strong> 8
<strong>解释:</strong> 边长长度为 1 的正方形不包含任何苹果。
但是边长为 2 的正方形包含 12 个苹果（如上图所示）。
周长为 2 * 4 = 8 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> neededApples = 13
<strong>输出:</strong> 16
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> neededApples = 1000000000
<strong>输出:</strong> 5040
</pre>

#### 提示:
* <code>1 <= neededApples <= 10<sup>15</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_perimeter(needed_apples: i64) -> i64 {
        let mut x = 1;

        while 4 * x * x * x + 6 * x * x + 2 * x < needed_apples {
            x += 1;
        }

        8 * x
    }
}
```
