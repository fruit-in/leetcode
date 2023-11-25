# 2106. Maximum Fruits Harvested After at Most K Steps
Fruits are available at some positions on an infinite x-axis. You are given a 2D integer array `fruits` where <code>fruits[i] = [position<sub>i</sub>, amount<sub>i</sub>]</code> depicts <code>amount<sub>i</sub></code> fruits at the position <code>position<sub>i</sub></code>. `fruits` is already **sorted** by <code>position<sub>i</sub></code> in **ascending order**, and each <code>position<sub>i</sub></code> is **unique**.

You are also given an integer `startPos` and an integer `k`. Initially, you are at the position `startPos`. From any position, you can either walk to the **left or right**. It takes **one step** to move **one unit** on the x-axis, and you can walk **at most** `k` steps in total. For every position you reach, you harvest all the fruits at that position, and the fruits will disappear from that position.

Return *the **maximum total number** of fruits you can harvest*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/11/21/1.png)
<pre>
<strong>Input:</strong> fruits = [[2,8],[6,3],[8,6]], startPos = 5, k = 4
<strong>Output:</strong> 9
<strong>Explanation:</strong>
The optimal way is to:
- Move right to position 6 and harvest 3 fruits
- Move right to position 8 and harvest 6 fruits
You moved 3 steps and harvested 3 + 6 = 9 fruits in total.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/11/21/2.png)
<pre>
<strong>Input:</strong> fruits = [[0,9],[4,1],[5,7],[6,2],[7,4],[10,9]], startPos = 5, k = 4
<strong>Output:</strong> 14
<strong>Explanation:</strong>
You can move at most k = 4 steps, so you cannot reach position 0 nor 10.
The optimal way is to:
- Harvest the 7 fruits at the starting position 5
- Move left to position 4 and harvest 1 fruit
- Move right to position 6 and harvest 2 fruits
- Move right to position 7 and harvest 4 fruits
You moved 1 + 3 = 4 steps and harvested 7 + 1 + 2 + 4 = 14 fruits in total.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/11/21/3.png)
<pre>
<strong>Input:</strong> fruits = [[0,3],[6,4],[8,5]], startPos = 3, k = 2
<strong>Output:</strong> 0
<strong>Explanation:</strong>
You can move at most k = 2 steps and cannot reach any position with fruits.
</pre>

#### Constraints:
* <code>1 <= fruits.length <= 10<sup>5</sup></code>
* `fruits[i].length == 2`
* <code>0 <= startPos, position<sub>i</sub> <= 2 * 10<sup>5</sup></code>
* <code>position<sub>i-1</sub> < position<sub>i</sub></code> for any `i > 0` (**0-indexed**)
* <code>1 <= amount<sub>i</sub> <= 10<sup>4</sup></code>
* <code>0 <= k <= 2 * 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let mut prefix_sum = vec![(-1, 0)];
        let mut ret = 0;

        for fruit in &fruits {
            if (fruit[0] - start_pos).abs() <= k {
                let amount = prefix_sum.last().unwrap().1;
                prefix_sum.push((fruit[0], fruit[1] + amount));
            }
        }

        for i in 1..prefix_sum.len() {
            if prefix_sum[i].0 < start_pos {
                let start = prefix_sum[i].0;
                let end = start_pos.max(k - start_pos + 2 * start);
                let j = prefix_sum.binary_search(&(end, i32::MAX)).unwrap_err() - 1;

                ret = ret.max(prefix_sum[j].1 - prefix_sum[i - 1].1);
            } else {
                let end = prefix_sum[i].0;
                let start = start_pos.min(2 * end - k - start_pos).max(0);
                let j = prefix_sum.binary_search(&(start, 0)).unwrap_err();

                ret = ret.max(prefix_sum[i].1 - prefix_sum[j - 1].1);
            }
        }

        ret
    }
}
```
