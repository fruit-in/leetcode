# 1801. 积压订单中的订单总数
给你一个二维整数数组 `orders` ，其中每个 <code>orders[i] = [price<sub>i</sub>, amount<sub>i</sub>, orderType<sub>i</sub>]</code> 表示有 <code>amount<sub>i</sub></code> 笔类型为 <code>orderType<sub>i</sub></code> 、价格为 <code>price<sub>i</sub></code> 的订单。

订单类型 <code>orderType<sub>i</sub></code> 可以分为两种：

* `0` 表示这是一批采购订单 `buy`
* `1` 表示这是一批销售订单 `sell`

注意，`orders[i]` 表示一批共计 <code>amount<sub>i</sub></code> 笔的独立订单，这些订单的价格和类型相同。对于所有有效的 `i` ，由 `orders[i]` 表示的所有订单提交时间均早于 `orders[i+1]` 表示的所有订单。

存在由未执行订单组成的 **积压订单** 。积压订单最初是空的。提交订单时，会发生以下情况：

* 如果该订单是一笔采购订单 `buy` ，则可以查看积压订单中价格 **最低** 的销售订单 `sell` 。如果该销售订单 `sell` 的价格 **低于或等于** 当前采购订单 `buy` 的价格，则匹配并执行这两笔订单，并将销售订单 `sell` 从积压订单中删除。否则，采购订单 `buy` 将会添加到积压订单中。
* 反之亦然，如果该订单是一笔销售订单 `sell` ，则可以查看积压订单中价格 **最高** 的采购订单 `buy` 。如果该采购订单 `buy` 的价格 **高于或等于** 当前销售订单 `sell` 的价格，则匹配并执行这两笔订单，并将采购订单 `buy` 从积压订单中删除。否则，销售订单 `sell` 将会添加到积压订单中。

输入所有订单后，返回积压订单中的 **订单总数** 。由于数字可能很大，所以需要返回对 <code>10<sup>9</sup> + 7</code> 取余的结果。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/03/11/ex1.png)
<pre>
<strong>输入:</strong> orders = [[10,5,0],[15,2,1],[25,1,1],[30,4,0]]
<strong>输出:</strong> 6
<strong>解释:</strong> 输入订单后会发生下述情况：
- 提交 5 笔采购订单，价格为 10 。没有销售订单，所以这 5 笔订单添加到积压订单中。
- 提交 2 笔销售订单，价格为 15 。没有采购订单的价格大于或等于 15 ，所以这 2 笔订单添加到积压订单中。
- 提交 1 笔销售订单，价格为 25 。没有采购订单的价格大于或等于 25 ，所以这 1 笔订单添加到积压订单中。
- 提交 4 笔采购订单，价格为 30 。前 2 笔采购订单与价格最低（价格为 15）的 2 笔销售订单匹配，从积压订单中删除这 2 笔销售订单。第 3 笔采购订单与价格最低的 1 笔销售订单匹配，销售订单价格为 25 ，从积压订单中删除这 1 笔销售订单。积压订单中不存在更多销售订单，所以第 4 笔采购订单需要添加到积压订单中。
最终，积压订单中有 5 笔价格为 10 的采购订单，和 1 笔价格为 30 的采购订单。所以积压订单中的订单总数为 6 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/03/11/ex2.png)
<pre>
<strong>输入:</strong> orders = [[7,1000000000,1],[15,3,0],[5,999999995,0],[5,1,1]]
<strong>输出:</strong> 999999984
<strong>解释:</strong> 输入订单后会发生下述情况：
- 提交 109 笔销售订单，价格为 7 。没有采购订单，所以这 109 笔订单添加到积压订单中。
- 提交 3 笔采购订单，价格为 15 。这些采购订单与价格最低（价格为 7 ）的 3 笔销售订单匹配，从积压订单中删除这 3 笔销售订单。
- 提交 999999995 笔采购订单，价格为 5 。销售订单的最低价为 7 ，所以这 999999995 笔订单添加到积压订单中。
- 提交 1 笔销售订单，价格为 5 。这笔销售订单与价格最高（价格为 5 ）的 1 笔采购订单匹配，从积压订单中删除这 1 笔采购订单。
最终，积压订单中有 (1000000000-3) 笔价格为 7 的销售订单，和 (999999995-1) 笔价格为 5 的采购订单。所以积压订单中的订单总数为 1999999991 ，等于 999999984 % (109 + 7) 。
</pre>

#### 提示:
* <code>1 <= orders.length <= 10<sup>5</sup></code>
* `orders[i].length == 3`
* <code>1 <= price<sub>i</sub>, amount<sub>i</sub> <= 10<sup>9</sup></code>
* <code>orderType<sub>i</sub></code> 为 `0` 或 `1`

## 题解 (Rust)

### 1. 题解
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
        let mut buys = BinaryHeap::new();
        let mut sells = BinaryHeap::new();

        for order in &orders {
            let price = order[0];
            let mut amount = order[1];
            let order_type = order[2];

            if order_type == 0 {
                while let Some(Reverse((p, a))) = sells.pop() {
                    if p > price {
                        sells.push(Reverse((p, a)));
                        break;
                    } else if amount >= a {
                        amount -= a;
                    } else {
                        sells.push(Reverse((p, a - amount)));
                        amount = 0;
                        break;
                    }
                }

                if amount > 0 {
                    buys.push((price, amount));
                }
            } else {
                while let Some((p, a)) = buys.pop() {
                    if p < price {
                        buys.push((p, a));
                        break;
                    } else if amount >= a {
                        amount -= a;
                    } else {
                        buys.push((p, a - amount));
                        amount = 0;
                        break;
                    }
                }

                if amount > 0 {
                    sells.push(Reverse((price, amount)));
                }
            }
        }

        (buys
            .iter()
            .map(|&(_, a)| a as i64)
            .chain(sells.iter().map(|&Reverse((_, a))| a as i64))
            .sum::<i64>()
            % 1_000_000_007) as i32
    }
}
```
