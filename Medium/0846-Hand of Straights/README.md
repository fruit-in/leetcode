# 846. Hand of Straights
Alice has a `hand` of cards, given as an array of integers.

Now she wants to rearrange the cards into groups so that each group is size `groupSize`, and consists of `groupSize` consecutive cards.

Return `true` if and only if she can.

**Note:** This question is the same as 1296: https://leetcode.com/problems/divide-array-in-sets-of-k-consecutive-numbers/

#### Example 1:
<pre>
<strong>Input:</strong> hand = [1,2,3,6,2,3,4,7,8], groupSize = 3
<strong>Output:</strong> true
<strong>Explanation:</strong> Alice's hand can be rearranged as [1,2,3],[2,3,4],[6,7,8]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> hand = [1,2,3,4,5], groupSize = 4
<strong>Output:</strong> false
<strong>Explanation:</strong> Alice's hand can't be rearranged into groups of 4.
</pre>

#### Constraints:
* `1 <= hand.length <= 10000`
* <code>0 <= hand[i] <= 10<sup>9</sup></code>
* `1 <= groupSize <= hand.length`

## Solutions (Rust)

### 1. HashMap
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
