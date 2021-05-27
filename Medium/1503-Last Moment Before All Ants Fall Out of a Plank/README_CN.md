# 1503. 所有蚂蚁掉下来前的最后一刻
有一块木板，长度为 `n` 个 **单位** 。一些蚂蚁在木板上移动，每只蚂蚁都以 **每秒一个单位** 的速度移动。其中，一部分蚂蚁向 **左** 移动，其他蚂蚁向 **右** 移动。

当两只向 **不同** 方向移动的蚂蚁在某个点相遇时，它们会同时改变移动方向并继续移动。假设更改方向不会花费任何额外时间。

而当蚂蚁在某一时刻 `t` 到达木板的一端时，它立即从木板上掉下来。

给你一个整数 `n` 和两个整数数组 `left` 以及 `right` 。两个数组分别标识向左或者向右移动的蚂蚁在 `t = 0` 时的位置。请你返回最后一只蚂蚁从木板上掉下来的时刻。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/06/17/ants.jpg)
<pre>
<strong>输入:</strong> n = 4, left = [4,3], right = [0,1]
<strong>输出:</strong> 4
<strong>解释:</strong> 如上图所示：
-下标 0 处的蚂蚁命名为 A 并向右移动。
-下标 1 处的蚂蚁命名为 B 并向右移动。
-下标 3 处的蚂蚁命名为 C 并向左移动。
-下标 4 处的蚂蚁命名为 D 并向左移动。
请注意，蚂蚁在木板上的最后时刻是 t = 4 秒，之后蚂蚁立即从木板上掉下来。（也就是说在 t = 4.0000000001 时，木板上没有蚂蚁）。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/06/17/ants2.jpg)
<pre>
<strong>输入:</strong> n = 7, left = [], right = [0,1,2,3,4,5,6,7]
<strong>输出:</strong> 7
<strong>解释:</strong> 所有蚂蚁都向右移动，下标为 0 的蚂蚁需要 7 秒才能从木板上掉落。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2020/06/17/ants3.jpg)
<pre>
<strong>输入:</strong> n = 7, left = [0,1,2,3,4,5,6,7], right = []
<strong>输出:</strong> 7
<strong>解释:</strong> 所有蚂蚁都向左移动，下标为 7 的蚂蚁需要 7 秒才能从木板上掉落。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> n = 9, left = [5], right = [4]
<strong>输出:</strong> 5
<strong>解释:</strong> t = 1 秒时，两只蚂蚁将回到初始位置，但移动方向与之前相反。
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> n = 6, left = [6], right = [0]
<strong>输出:</strong> 6
</pre>

#### 提示:
* `1 <= n <= 10^4`
* `0 <= left.length <= n + 1`
* `0 <= left[i] <= n`
* `0 <= right.length <= n + 1`
* `0 <= right[i] <= n`
* `1 <= left.length + right.length <= n + 1`
* `left` 和 `right` 中的所有值都是唯一的，并且每个值 **只能出现在二者之一** 中。

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer} n
# @param {Integer[]} left
# @param {Integer[]} right
# @return {Integer}
def get_last_moment(n, left, right)
  (left + right.map { |x| n - x }).max
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        left.into_iter()
            .chain(right.into_iter().map(|x| n - x))
            .max()
            .unwrap()
    }
}
```
