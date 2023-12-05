# 1744. Can You Eat Your Favorite Candy on Your Favorite Day?
You are given a **(0-indexed)** array of positive integers `candiesCount` where `candiesCount[i]` represents the number of candies of the <code>i<sup>th</sup></code> type you have. You are also given a 2D array `queries` where <code>queries[i] = [favoriteType<sub>i</sub>, favoriteDay<sub>i</sub>, dailyCap<sub>i</sub>]</code>.

You play a game with the following rules:

* You start eating candies on day **`0`**.
* You **cannot** eat **any** candy of type `i` unless you have eaten **all** candies of type `i - 1`.
* You must eat **at least one** candy per day until you have eaten all the candies.

Construct a boolean array `answer` such that `answer.length == queries.length` and `answer[i]` is `true` if you can eat a candy of type <code>favoriteType<sub>i</sub></code> on day <code>favoriteDay<sub>i</sub></code> without eating **more than** <code>dailyCap<sub>i</sub></code> candies on **any** day, and `false` otherwise. Note that you can eat different types of candy on the same day, provided that you follow rule 2.

Return *the constructed array* `answer`.

#### Example 1:
<pre>
<strong>Input:</strong> candiesCount = [7,4,5,3,8], queries = [[0,2,2],[4,2,4],[2,13,1000000000]]
<strong>Output:</strong> [true,false,true]
<strong>Explanation:</strong>
1- If you eat 2 candies (type 0) on day 0 and 2 candies (type 0) on day 1, you will eat a candy of type 0 on day 2.
2- You can eat at most 4 candies each day.
   If you eat 4 candies every day, you will eat 4 candies (type 0) on day 0 and 4 candies (type 0 and type 1) on day 1.
   On day 2, you can only eat 4 candies (type 1 and type 2), so you cannot eat a candy of type 4 on day 2.
3- If you eat 1 candy each day, you will eat a candy of type 2 on day 13.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> candiesCount = [5,2,6,4,1], queries = [[3,1,2],[4,10,3],[3,10,100],[4,100,30],[1,3,1]]
<strong>Output:</strong> [false,true,true,false,false]
</pre>

#### Constraints:
* <code>1 <= candiesCount.length <= 10<sup>5</sup></code>
* <code>1 <= candiesCount[i] <= 10<sup>5</sup></code>
* <code>1 <= queries.length <= 10<sup>5</sup></code>
* `queries[i].length == 3`
* <code>0 <= favoriteType<sub>i</sub> < candiesCount.length</code>
* <code>0 <= favoriteDay<sub>i</sub> <= 10<sup>9</sup></code>
* <code>1 <= dailyCap<sub>i</sub> <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
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
