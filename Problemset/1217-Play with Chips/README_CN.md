# 1217. 玩筹码
数轴上放置了一些筹码，每个筹码的位置存在数组 ```chips``` 当中。

你可以对 **任何筹码** 执行下面两种操作之一（**不限操作次数**，0 次也可以）：
* 将第 ```i``` 个筹码向左或者右移动 2 个单位，代价为 **0**。
* 将第 ```i``` 个筹码向左或者右移动 1 个单位，代价为 **1**。

#### 示例 1:
<pre>
<strong>输入:</strong> chips = [1,2,3]
<strong>输出:</strong> 1
<strong>解释:</strong> 第二个筹码移动到位置三的代价是 1，第一个筹码移动到位置三的代价是 0，总代价为 1。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> chips = [2,2,2,3,3]
<strong>输出:</strong> 2
<strong>解释:</strong> 第四和第五个筹码移动到位置二的代价都是 1，所以最小总代价为 2。
</pre>

#### 提示:
* ```1 <= chips.length <= 100```
* ```1 <= chips[i] <= 10^9```

## 题解 (Rust)

### 1. 分别对奇数、偶数计数
```Rust
impl Solution {
    pub fn min_cost_to_move_chips(chips: Vec<i32>) -> i32 {
        let odd_cnt = chips.iter().filter(|&x| x % 2 == 1).count();
        odd_cnt.min(chips.len() - odd_cnt) as i32
    }
}
```
