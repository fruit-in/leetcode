# 846. 一手顺子
爱丽丝有一手（`hand`）由整数数组给定的牌。

现在她想把牌重新排列成组，使得每个组的大小都是 `W`，且由 `W` 张连续的牌组成。

如果她可以完成分组就返回 `true`，否则返回 `false`。

**注意:** 此题目与 1296 重复：https://leetcode-cn.com/problems/divide-array-in-sets-of-k-consecutive-numbers/

#### 示例 1:
<pre>
<strong>输入:</strong> hand = [1,2,3,6,2,3,4,7,8], W = 3
<strong>输出:</strong> true
<strong>解释:</strong> 爱丽丝的手牌可以被重新排列为 [1,2,3]，[2,3,4]，[6,7,8]。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> hand = [1,2,3,4,5], W = 4
<strong>输出:</strong> false
<strong>解释:</strong> 爱丽丝的手牌无法被重新排列成几个大小为 4 的组。
</pre>

#### 提示:
* `1 <= hand.length <= 10000`
* `0 <= hand[i] <= 10^9`
* `1 <= W <= hand.length`

## 题解 (Rust)

### 1. 哈希表
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
        let mut needs: HashMap<i32, Vec<i32>> = HashMap::new();
        hand.sort_unstable();

        for x in hand {
            if let Some(v) = needs.get_mut(&(x - 1)) {
                match v.pop() {
                    Some(1) => (),
                    Some(y) => needs.entry(x).or_insert(vec![]).push(y - 1),
                    None => needs.entry(x).or_insert(vec![]).push(group_size - 1),
                }
            } else if group_size > 1 {
                needs.entry(x).or_insert(vec![]).push(group_size - 1);
            }
        }

        needs.values().all(|v| v.is_empty())
    }
}
```
