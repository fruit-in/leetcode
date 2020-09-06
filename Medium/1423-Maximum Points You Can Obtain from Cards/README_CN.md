# 1423. 可获得的最大点数
几张卡牌 **排成一行**，每张卡牌都有一个对应的点数。点数由整数数组 `cardPoints` 给出。

每次行动，你可以从行的开头或者末尾拿一张卡牌，最终你必须正好拿 `k` 张卡牌。

你的点数就是你拿到手中的所有卡牌的点数之和。

给你一个整数数组 `cardPoints` 和整数 `k`，请你返回可以获得的最大点数。

#### 示例 1:
<pre>
<strong>输入:</strong> cardPoints = [1,2,3,4,5,6,1], k = 3
<strong>输出:</strong> 12
<strong>解释:</strong> 第一次行动，不管拿哪张牌，你的点数总是 1 。但是，先拿最右边的卡牌将会最大化你的可获得点数。最优策略是拿右边的三张牌，最终点数为 1 + 6 + 5 = 12 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> cardPoints = [2,2,2], k = 2
<strong>输出:</strong> 4
<strong>解释:</strong> 无论你拿起哪两张卡牌，可获得的点数总是 4 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> cardPoints = [9,7,7,9,7,7,9], k = 7
<strong>输出:</strong> 55
<strong>解释:</strong> 你必须拿起所有卡牌，可以获得的点数为所有卡牌的点数之和。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> cardPoints = [1,1000,1], k = 1
<strong>输出:</strong> 1
<strong>解释:</strong> 你无法拿到中间那张卡牌，所以可以获得的最大点数为 1 。
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> cardPoints = [1,79,80,1,1,1,200,1], k = 3
<strong>输出:</strong> 202
</pre>

#### 提示:
* `1 <= cardPoints.length <= 10^5`
* `1 <= cardPoints[i] <= 10^4`
* `1 <= k <= cardPoints.length`

## 题解 (Rust)

### 1. 滑动窗口
```Rust
impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut score = card_points.iter().take(k).sum::<i32>();
        let mut ret = score;

        for i in 1..=k {
            score += card_points[card_points.len() - i] - card_points[k - i];
            ret = ret.max(score);
        }

        ret
    }
}
```
