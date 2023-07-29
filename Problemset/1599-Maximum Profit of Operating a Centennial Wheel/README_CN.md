# 1599. 经营摩天轮的最大利润
你正在经营一座摩天轮，该摩天轮共有 **4 个座舱** ，每个座舱 **最多可以容纳 4 位游客** 。你可以 **逆时针** 轮转座舱，但每次轮转都需要支付一定的运行成本 `runningCost` 。摩天轮每次轮转都恰好转动 1 / 4 周。

给你一个长度为 `n` 的数组 `customers` ， `customers[i]` 是在第 `i` 次轮转（下标从 0 开始）之前到达的新游客的数量。这也意味着你必须在新游客到来前轮转 `i` 次。每位游客在登上离地面最近的座舱前都会支付登舱成本 `boardingCost` ，一旦该座舱再次抵达地面，他们就会离开座舱结束游玩。

你可以随时停下摩天轮，即便是 **在服务所有游客之前** 。如果你决定停止运营摩天轮，为了保证所有游客安全着陆，**将免费进行所有后续轮转** 。注意，如果有超过 4 位游客在等摩天轮，那么只有 4 位游客可以登上摩天轮，其余的需要等待 **下一次轮转** 。

返回最大化利润所需执行的 **最小轮转次数** 。 如果不存在利润为正的方案，则返回 `-1` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/09/09/wheeldiagram12.png)
<pre>
<strong>输入:</strong> customers = [8,3], boardingCost = 5, runningCost = 6
<strong>输出:</strong> 3
<strong>解释:</strong> 座舱上标注的数字是该座舱的当前游客数。
1. 8 位游客抵达，4 位登舱，4 位等待下一舱，摩天轮轮转。当前利润为 4 * $5 - 1 * $6 = $14 。
2. 3 位游客抵达，4 位在等待的游客登舱，其他 3 位等待，摩天轮轮转。当前利润为 8 * $5 - 2 * $6 = $28 。
3. 最后 3 位游客登舱，摩天轮轮转。当前利润为 11 * $5 - 3 * $6 = $37 。
轮转 3 次得到最大利润，最大利润为 $37 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> customers = [10,9,6], boardingCost = 6, runningCost = 4
<strong>输出:</strong> 7
<strong>解释:</strong>
1. 10 位游客抵达，4 位登舱，6 位等待下一舱，摩天轮轮转。当前利润为 4 * $6 - 1 * $4 = $20 。
2. 9 位游客抵达，4 位登舱，11 位等待（2 位是先前就在等待的，9 位新加入等待的），摩天轮轮转。当前利润为 8 * $6 - 2 * $4 = $40 。
3. 最后 6 位游客抵达，4 位登舱，13 位等待，摩天轮轮转。当前利润为 12 * $6 - 3 * $4 = $60 。
4. 4 位登舱，9 位等待，摩天轮轮转。当前利润为 * $6 - 4 * $4 = $80 。
5. 4 位登舱，5 位等待，摩天轮轮转。当前利润为 20 * $6 - 5 * $4 = $100 。
6. 4 位登舱，1 位等待，摩天轮轮转。当前利润为 24 * $6 - 6 * $4 = $120 。
7. 1 位登舱，摩天轮轮转。当前利润为 25 * $6 - 7 * $4 = $122 。
轮转 7 次得到最大利润，最大利润为$122 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> customers = [3,4,0,5,1], boardingCost = 1, runningCost = 92
<strong>输出:</strong> -1
<strong>解释:</strong>
1. 3 位游客抵达，3 位登舱，0 位等待，摩天轮轮转。当前利润为 3 * $1 - 1 * $92 = -$89 。
2. 4 位游客抵达，4 位登舱，0 位等待，摩天轮轮转。当前利润为 is 7 * $1 - 2 * $92 = -$177 。
3. 0 位游客抵达，0 位登舱，0 位等待，摩天轮轮转。当前利润为 7 * $1 - 3 * $92 = -$269 。
4. 5 位游客抵达，4 位登舱，1 位等待，摩天轮轮转。当前利润为 12 * $1 - 4 * $92 = -$356 。
5. 1 位游客抵达，2 位登舱，0 位等待，摩天轮轮转。当前利润为 13 * $1 - 5 * $92 = -$447 。
利润永不为正，所以返回 -1 。
</pre>

#### 提示:
* `n == customers.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* `0 <= customers[i] <= 50`
* `1 <= boardingCost, runningCost <= 100`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_operations_max_profit(
        customers: Vec<i32>,
        boarding_cost: i32,
        running_cost: i32,
    ) -> i32 {
        if 4 * boarding_cost <= running_cost {
            return -1;
        }

        let mut i = 0;
        let mut customer = 0;
        let mut profit = 0;
        let mut max_profit = 0;
        let mut ret = -1;

        while i < customers.len() || customer > 0 {
            customer += customers.get(i).unwrap_or(&0);
            profit += customer.min(4) * boarding_cost - running_cost;
            customer -= customer.min(4);
            i += 1;

            if profit > max_profit {
                max_profit = profit;
                ret = i as i32;
            }
        }

        ret
    }
}
```
