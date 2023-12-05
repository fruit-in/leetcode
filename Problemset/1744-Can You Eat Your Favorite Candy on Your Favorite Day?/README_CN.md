# 1744. 你能在你最喜欢的那天吃到你最喜欢的糖果吗？
给你一个下标从 **0** 开始的正整数数组 `candiesCount` ，其中 `candiesCount[i]` 表示你拥有的第 `i` 类糖果的数目。同时给你一个二维数组 `queries` ，其中 <code>queries[i] = [favoriteType<sub>i</sub>, favoriteDay<sub>i</sub>, dailyCap<sub>i</sub>]</code> 。

你按照如下规则进行一场游戏：

* 你从第 `0` 天开始吃糖果。
* 你在吃完 所有 第 `i - 1` 类糖果之前，**不能** 吃任何一颗第 `i` 类糖果。
* 在吃完所有糖果之前，你必须每天 **至少** 吃 **一颗** 糖果。

请你构建一个布尔型数组 `answer` ，用以给出 `queries` 中每一项的对应答案。此数组满足：

* `answer.length == queries.length` 。`answer[i]` 是 `queries[i]` 的答案。
* `answer[i]` 为 `true` 的条件是：在每天吃 **不超过** <code>dailyCap<sub>i</sub></code> 颗糖果的前提下，你可以在第 <code>favoriteDay<sub>i</sub></code> 天吃到第 <code>favoriteType<sub>i</sub></code> 类糖果；否则 `answer[i]` 为 `false` 。

注意，只要满足上面 3 条规则中的第二条规则，你就可以在同一天吃不同类型的糖果。

请你返回得到的数组 `answer` 。

#### 示例 1:
<pre>
<strong>输入:</strong> candiesCount = [7,4,5,3,8], queries = [[0,2,2],[4,2,4],[2,13,1000000000]]
<strong>输出:</strong> [true,false,true]
<strong>提示:</strong>
1- 在第 0 天吃 2 颗糖果(类型 0），第 1 天吃 2 颗糖果（类型 0），第 2 天你可以吃到类型 0 的糖果。
2- 每天你最多吃 4 颗糖果。即使第 0 天吃 4 颗糖果（类型 0），第 1 天吃 4 颗糖果（类型 0 和类型 1），你也没办法在第 2 天吃到类型 4 的糖果。换言之，你没法在每天吃 4 颗糖果的限制下在第 2 天吃到第 4 类糖果。
3- 如果你每天吃 1 颗糖果，你可以在第 13 天吃到类型 2 的糖果。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> candiesCount = [5,2,6,4,1], queries = [[3,1,2],[4,10,3],[3,10,100],[4,100,30],[1,3,1]]
<strong>输出:</strong> [false,true,true,false,false]
</pre>

#### 提示:
* <code>1 <= candiesCount.length <= 10<sup>5</sup></code>
* <code>1 <= candiesCount[i] <= 10<sup>5</sup></code>
* <code>1 <= queries.length <= 10<sup>5</sup></code>
* `queries[i].length == 3`
* <code>0 <= favoriteType<sub>i</sub> < candiesCount.length</code>
* <code>0 <= favoriteDay<sub>i</sub> <= 10<sup>9</sup></code>
* <code>1 <= dailyCap<sub>i</sub> <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut prefix_sum = candies_count
            .into_iter()
            .map(|x| x as i64)
            .collect::<Vec<_>>();
        let mut answer = vec![true; queries.len()];

        for i in 1..prefix_sum.len() {
            prefix_sum[i] += prefix_sum[i - 1];
        }

        for i in 0..answer.len() {
            let favorite_type = queries[i][0] as usize;
            let favorite_day = queries[i][1] as i64;
            let daily_cap = queries[i][2] as i64;

            answer[i] = prefix_sum[favorite_type] > favorite_day
                && (favorite_type == 0
                    || (favorite_day + 1) * daily_cap > prefix_sum[favorite_type - 1]);
        }

        answer
    }
}
```
