# 2212. 射箭比赛中的最大得分
Alice 和 Bob 是一场射箭比赛中的对手。比赛规则如下：

1. Alice 先射 `numArrows` 支箭，然后 Bob 也射 `numArrows` 支箭。
2. 分数按下述规则计算：
    1. 箭靶有若干整数计分区域，范围从 `0` 到 `11` （含 `0` 和 `11`）。
    2. 箭靶上每个区域都对应一个得分 `k`（范围是 `0` 到 `11`），Alice 和 Bob 分别在得分 `k` 区域射中 <code>a<sub>k</sub></code> 和 <code>b<sub>k</sub></code> 支箭。如果 <code>a<sub>k</sub> >= b<sub>k</sub></code> ，那么 Alice 得 `k` 分。如果 <code>a<sub>k</sub> < b<sub>k</sub></code> ，则 Bob 得 `k` 分
    3. 如果 <code>a<sub>k</sub> == b<sub>k</sub> == 0</code> ，那么无人得到 `k` 分。

* 例如，Alice 和 Bob 都向计分为 `11` 的区域射 `2` 支箭，那么 Alice 得 `11` 分。如果 Alice 向计分为 `11` 的区域射 `0` 支箭，但 Bob 向同一个区域射 `2` 支箭，那么 Bob 得 `11` 分。

给你整数 `numArrows` 和一个长度为 `12` 的整数数组 `aliceArrows` ，该数组表示 Alice 射中 `0` 到 `11` 每个计分区域的箭数量。现在，Bob 想要尽可能 **最大化** 他所能获得的总分。

返回数组 `bobArrows` ，该数组表示 Bob 射中 `0` 到 `11` **每个** 计分区域的箭数量。且 `bobArrows` 的总和应当等于 `numArrows` 。

如果存在多种方法都可以使 Bob 获得最大总分，返回其中 **任意一种** 即可。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/02/24/ex1.jpg)
<pre>
<strong>输入:</strong> numArrows = 9, aliceArrows = [1,1,0,1,0,0,2,1,0,1,2,0]
<strong>输出:</strong> [0,0,0,0,1,1,0,0,1,2,3,1]
<strong>解释:</strong> 上表显示了比赛得分情况。
Bob 获得总分 4 + 5 + 8 + 9 + 10 + 11 = 47 。
可以证明 Bob 无法获得比 47 更高的分数。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/02/24/ex2new.jpg)
<pre>
<strong>输入:</strong> numArrows = 3, aliceArrows = [0,0,1,0,0,0,0,0,0,0,0,2]
<strong>输出:</strong> [0,0,0,0,0,0,0,0,1,1,1,0]
<strong>解释:</strong> 上表显示了比赛得分情况。
Bob 获得总分 8 + 9 + 10 = 27 。
可以证明 Bob 无法获得比 27 更高的分数。
</pre>

#### 提示:
* <code>1 <= numArrows <= 10<sup>5</sup></code>
* `aliceArrows.length == bobArrows.length == 12`
* `0 <= aliceArrows[i], bobArrows[i] <= numArrows`
* `sum(aliceArrows[i]) == numArrows`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn maximum_bob_points(num_arrows: i32, alice_arrows: Vec<i32>) -> Vec<i32> {
        let mut max_points = 0;
        let mut bob_arrows = vec![];

        for x in 1..2_i32.pow(12) {
            let mut points = 0;
            let mut arrows = vec![0; 12];
            let mut arrows_sum = 0;

            for i in 0..12 {
                if x & (1 << i) != 0 {
                    points += i;
                    arrows[i] = alice_arrows[i] + 1;
                    arrows_sum += arrows[i];
                }
            }

            if points > max_points && arrows_sum <= num_arrows {
                max_points = points;
                arrows[0] += num_arrows - arrows_sum;
                bob_arrows = arrows;
            }
        }

        bob_arrows
    }
}
```
