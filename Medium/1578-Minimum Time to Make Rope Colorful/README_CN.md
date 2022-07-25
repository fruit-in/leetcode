# 1578. 使绳子变成彩色的最短时间
Alice 把 `n` 个气球排列在一根绳子上。给你一个下标从 **0** 开始的字符串 `colors` ，其中 `colors[i]` 是第 `i` 个气球的颜色。

Alice 想要把绳子装扮成 **彩色** ，且她不希望两个连续的气球涂着相同的颜色，所以她喊来 Bob 帮忙。Bob 可以从绳子上移除一些气球使绳子变成 **彩色** 。给你一个下标从 **0** 开始的整数数组 `neededTime` ，其中 `neededTime[i]` 是 Bob 从绳子上移除第 `i` 个气球需要的时间（以秒为单位）。

返回 Bob 使绳子变成 **彩色** 需要的 **最少时间** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/12/13/ballon1.jpg)
<pre>
<strong>输入:</strong> colors = "abaac", neededTime = [1,2,3,4,5]
<strong>输出:</strong> 3
<strong>解释:</strong> 在上图中，'a' 是蓝色，'b' 是红色且 'c' 是绿色。
Bob 可以移除下标 2 的蓝色气球。这将花费 3 秒。
移除后，不存在两个连续的气球涂着相同的颜色。总时间 = 3 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/12/13/balloon2.jpg)
<pre>
<strong>输入:</strong> colors = "abc", neededTime = [1,2,3]
<strong>输出:</strong> 0
<strong>解释:</strong> 绳子已经是彩色的，Bob 不需要从绳子上移除任何气球。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/12/13/balloon3.jpg)
<pre>
<strong>输入:</strong> colors = "aabaa", neededTime = [1,2,3,4,1]
<strong>输出:</strong> 2
<strong>解释:</strong> Bob 会移除下标 0 和下标 4 处的气球。这两个气球各需要 1 秒来移除。
移除后，不存在两个连续的气球涂着相同的颜色。总时间 = 1 + 1 = 2 。
</pre>

#### 提示:
* `n == colors.length == neededTime.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= neededTime[i] <= 10<sup>4</sup></code>
* `colors` 仅由小写英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
impl 题解 {
    pub fn min_cost(s: String, cost: Vec<i32>) -> i32 {
        let s = s.as_bytes();
        let mut prev = 0;
        let mut ret = 0;

        for i in 1..s.len() {
            if s[i] == s[prev] {
                if cost[i] < cost[prev] {
                    ret += cost[i];
                    continue;
                }
                ret += cost[prev];
            }
            prev = i;
        }

        ret
    }
}
```
