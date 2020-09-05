# 1375. 灯泡开关 III
房间中有 `n` 枚灯泡，编号从 `1` 到 `n`，自左向右排成一排。最初，所有的灯都是关着的。

在 *k*  时刻（ *k* 的取值范围是 `0` 到 `n - 1`），我们打开 `light[k]` 这个灯。

灯的颜色要想 **变成蓝色** 就必须同时满足下面两个条件：
* 灯处于打开状态。
* 排在它之前（左侧）的所有灯也都处于打开状态。

请返回能够让 **所有开着的** 灯都 **变成蓝色** 的时刻 **数目** 。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/03/08/sample_2_1725.png)
<pre>
<strong>输入:</strong> light = [2,1,3,5,4]
<strong>输出:</strong> 3
<strong>解释:</strong> 所有开着的灯都变蓝的时刻分别是 1，2 和 4 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> light = [3,2,4,1,5]
<strong>输出:</strong> 2
<strong>解释:</strong> 所有开着的灯都变蓝的时刻分别是 3 和 4（index-0）。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> light = [4,1,2,3]
<strong>输出:</strong> 1
<strong>解释:</strong> 所有开着的灯都变蓝的时刻是 3（index-0）。
第 4 个灯在时刻 3 变蓝。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> light = [2,1,4,3,6,5]
<strong>输出:</strong> 3
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> light = [1,2,3,4,5,6]
<strong>输出:</strong> 6
</pre>

#### 提示:
* `n == light.length`
* `1 <= n <= 5 * 10^4`
* `light` 是 `[1, 2, ..., n]` 的一个排列。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn num_times_all_blue(light: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut ret = 0;

        for k in 0..light.len() {
            max = max.max(light[k]);
            if max == k as i32 + 1 {
                ret += 1;
            }
        }

        ret
    }
}
```
