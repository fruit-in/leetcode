# 1833. 雪糕的最大数量
夏日炎炎，小男孩 Tony 想买一些雪糕消消暑。

商店中新到 `n` 支雪糕，用长度为 `n` 的数组 `costs` 表示雪糕的定价，其中 `costs[i]` 表示第 `i` 支雪糕的现金价格。Tony 一共有 `coins` 现金可以用于消费，他想要买尽可能多的雪糕。

给你价格数组 `costs` 和现金量 `coins` ，请你计算并返回 Tony 用 `coins` 现金能够买到的雪糕的 **最大数量** 。

**注意:** Tony 可以按任意顺序购买雪糕。

#### 示例 1:
<pre>
<strong>输入:</strong> costs = [1,3,2,4,1], coins = 7
<strong>输出:</strong> 4
<strong>解释:</strong> Tony 可以买下标为 0、1、2、4 的雪糕，总价为 1 + 3 + 2 + 1 = 7
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> costs = [10,6,8,7,7,8], coins = 5
<strong>输出:</strong> 0
<strong>解释:</strong> Tony 没有足够的钱买任何一支雪糕。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> costs = [1,6,3,1,2,5], coins = 20
<strong>输出:</strong> 6
<strong>解释:</strong> Tony 可以买下所有的雪糕，总价为 1 + 6 + 3 + 1 + 2 + 5 = 18 。
</pre>

#### 提示:
* `costs.length == n`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= costs[i] <= 10<sup>5</sup></code>
* <code>1 <= coins <= 10<sup>8</sup></code>

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
        costs.sort_unstable();

        costs
            .iter()
            .take_while(|&x| {
                coins -= x;
                coins >= 0
            })
            .count() as i32
    }
}
```
