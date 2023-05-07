# 1648. 销售价值减少的颜色球
你有一些球的库存 `inventory` ，里面包含着不同颜色的球。一个顾客想要 **任意颜色** 总数为 `orders` 的球。

这位顾客有一种特殊的方式衡量球的价值：每个球的价值是目前剩下的 **同色球** 的数目。比方说还剩下 `6` 个黄球，那么顾客买第一个黄球的时候该黄球的价值为 `6` 。这笔交易以后，只剩下 `5` 个黄球了，所以下一个黄球的价值为 `5` （也就是球的价值随着顾客购买同色球是递减的）

给你整数数组 `inventory` ，其中 `inventory[i]` 表示第 `i` 种颜色球一开始的数目。同时给你整数 `orders` ，表示顾客总共想买的球数目。你可以按照 **任意顺序** 卖球。

请你返回卖了 `orders` 个球以后 **最大** 总价值之和。由于答案可能会很大，请你返回答案对 <code>10<sup>9</sup> + 7</code> **取余数** 的结果。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/11/05/jj.gif)
<pre>
<strong>输入:</strong> inventory = [2,5], orders = 4
<strong>输出:</strong> 14
<strong>解释:</strong> 卖 1 个第一种颜色的球（价值为 2 )，卖 3 个第二种颜色的球（价值为 5 + 4 + 3）。
最大总和为 2 + 5 + 4 + 3 = 14 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> inventory = [3,5], orders = 6
<strong>输出:</strong> 19
<strong>解释:</strong> 卖 2 个第一种颜色的球（价值为 3 + 2），卖 4 个第二种颜色的球（价值为 5 + 4 + 3 + 2）。
最大总和为 3 + 2 + 5 + 4 + 3 + 2 = 19 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> inventory = [2,8,4,10,6], orders = 20
<strong>输出:</strong> 110
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> inventory = [1000000000], orders = 1000000000
<strong>输出:</strong> 21
<strong>解释:</strong> 卖 1000000000 次第一种颜色的球，总价值为 500000000500000000 。 500000000500000000 对 109 + 7 取余为 21 。
</pre>

#### 提示:
* <code>1 <= inventory.length <= 10<sup>5</sup></code>
* <code>1 <= inventory[i] <= 10<sup>9</sup></code>
* <code>1 <= orders <= min(sum(inventory[i]), 10<sup>9</sup>)</code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn max_profit(inventory: Vec<i32>, orders: i32) -> i32 {
        let mut orders = orders as i64;
        let mut count = HashMap::new();
        let mut ret = 0;

        for &balls in &inventory {
            count
                .entry(balls as i64)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }

        let mut heap = count.into_iter().collect::<BinaryHeap<_>>();

        while orders > 0 {
            let (b0, c0) = heap.pop().unwrap();
            let (b1, c1) = heap.pop().unwrap_or((0, 0));

            if (b0 - b1) * c0 < orders {
                orders -= (b0 - b1) * c0;
                heap.push((b1, c0 + c1));
                ret = (ret + (b0 - b1) * (b0 + b1 + 1) * c0 / 2) % 1_000_000_007;
            } else {
                let (x, y) = (orders / c0, orders % c0);
                orders = 0;
                ret = (ret + (b0 * 2 - x + 1) * x * c0 / 2 + (b0 - x) * y) % 1_000_000_007;
            }
        }

        ret as i32
    }
}
```
