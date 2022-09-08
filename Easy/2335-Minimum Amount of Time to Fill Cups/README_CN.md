# 2335. 装满杯子需要的最短总时长
现有一台饮水机，可以制备冷水、温水和热水。每秒钟，可以装满 `2` 杯 **不同** 类型的水或者 `1` 杯任意类型的水。

给你一个下标从 **0** 开始、长度为 `3` 的整数数组 `amount` ，其中 `amount[0]`、`amount[1]` 和 `amount[2]` 分别表示需要装满冷水、温水和热水的杯子数量。返回装满所有杯子所需的 **最少** 秒数。

#### 示例 1:
<pre>
<strong>输入:</strong> amount = [1,4,2]
<strong>输出:</strong> 4
<strong>解释:</strong> 下面给出一种方案：
第 1 秒：装满一杯冷水和一杯温水。
第 2 秒：装满一杯温水和一杯热水。
第 3 秒：装满一杯温水和一杯热水。
第 4 秒：装满一杯温水。
可以证明最少需要 4 秒才能装满所有杯子。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> amount = [5,4,4]
<strong>输出:</strong> 7
<strong>解释:</strong> 下面给出一种方案：
第 1 秒：装满一杯冷水和一杯热水。
第 2 秒：装满一杯冷水和一杯温水。
第 3 秒：装满一杯冷水和一杯温水。
第 4 秒：装满一杯温水和一杯热水。
第 5 秒：装满一杯冷水和一杯热水。
第 6 秒：装满一杯冷水和一杯温水。
第 7 秒：装满一杯热水。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> amount = [5,0,0]
<strong>输出:</strong> 5
<strong>解释:</strong> 每秒装满一杯冷水。
</pre>

#### 提示:
* `amount.length == 3`
* `0 <= amount[i] <= 100`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn fill_cups(amount: Vec<i32>) -> i32 {
        let mut amount = amount;
        amount.sort_unstable();

        if amount[0] + amount[1] < amount[2] {
            amount[2]
        } else {
            (amount.iter().sum::<i32>() + 1) / 2
        }
    }
}
```
