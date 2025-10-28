# 335. 路径交叉
给你一个整数数组 `distance` 。

从 **X-Y** 平面上的点 `(0,0)` 开始，先向北移动 `distance[0]` 米，然后向西移动 `distance[1]` 米，向南移动 `distance[2]` 米，向东移动 `distance[3]` 米，持续移动。也就是说，每次移动后你的方位会发生逆时针变化。

判断你所经过的路径是否相交。如果相交，返回 `true` ；否则，返回 `false` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/03/14/selfcross1-plane.jpg)
<pre>
<strong>输入:</strong> distance = [2,1,1,2]
<strong>输出:</strong> true
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/03/14/selfcross2-plane.jpg)
<pre>
<strong>输入:</strong> distance = [1,2,3,4]
<strong>输出:</strong> false
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/03/14/selfcross3-plane.jpg)
<pre>
<strong>输入:</strong> distance = [1,1,1,1]
<strong>输出:</strong> true
</pre>

#### 提示:
* <code>1 <= distance.length <= 10<sup>5</sup></code>
* <code>1 <= distance[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn is_self_crossing(mut distance: Vec<i32>) -> bool {
        let mut i = 2;

        while i < distance.len() {
            if distance[i] <= distance[i - 2] {
                let mut tmp = distance[i - 2];
                if i > 3 {
                    tmp -= distance[i - 4];
                }
                if i > 2 && distance[i] >= tmp {
                    distance[i - 1] -= distance[i - 3];
                }
                break;
            }

            i += 1;
        }

        while i < distance.len() - 1 {
            i += 1;

            if distance[i] >= distance[i - 2] {
                return true;
            }
        }

        false
    }
}
```
