# 1217. Play with Chips
There are some chips, and the i-th chip is at position ```chips[i]```.

You can perform any of the two following types of moves **any number of times** (possibly zero) **on any chip**:
* Move the ```i```-th chip by 2 units to the left or to the right with a cost of **0**.
* Move the ```i```-th chip by 1 unit to the left or to the right with a cost of **1**.

There can be two or more chips at the same position initially.

Return the minimum cost needed to move all the chips to the same position (any position).

#### Example 1:
<pre>
<strong>Input:</strong> chips = [1,2,3]
<strong>Output:</strong> 1
<strong>Explanation:</strong> Second chip will be moved to positon 3 with cost 1.
First chip will be moved to position 3 with cost 0.
Total cost is 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> chips = [2,2,2,3,3]
<strong>Output:</strong> 2
<strong>Explanation:</strong> Both fourth and fifth chip will be moved to position two with cost 1.
Total minimum cost will be 2.
</pre>

#### Constraints:
* ```1 <= chips.length <= 100```
* ```1 <= chips[i] <= 10^9```

## Solutions (Rust)

### 1. Count Odd Numbers & Even Numbers
```Rust
impl Solution {
    pub fn min_cost_to_move_chips(chips: Vec<i32>) -> i32 {
        let odd_cnt = chips.iter().filter(|&x| x % 2 == 1).count();
        odd_cnt.min(chips.len() - odd_cnt) as i32
    }
}
```
