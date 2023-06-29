# 2260. Minimum Consecutive Cards to Pick Up
You are given an integer array `cards` where `cards[i]` represents the **value** of the <code>i<sup>th</sup></code> card. A pair of cards are **matching** if the cards have the **same** value.

Return *the **minimum** number of **consecutive** cards you have to pick up to have a pair of **matching** cards among the picked cards*. If it is impossible to have matching cards, return `-1`.

#### Example 1:
<pre>
<strong>Input:</strong> cards = [3,4,2,3,4,7]
<strong>Output:</strong> 4
<strong>Explanation:</strong> We can pick up the cards [3,4,2,3] which contain a matching pair of cards with value 3. Note that picking up the cards [4,2,3,4] is also optimal.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> cards = [1,0,5,3]
<strong>Output:</strong> -1
<strong>Explanation:</strong> There is no way to pick up a set of consecutive cards that contain a pair of matching cards.
</pre>

#### Constraints:
* <code>1 <= cards.length <= 10<sup>5</sup></code>
* <code>0 <= cards[i] <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
        let mut indices = HashMap::new();
        let mut ret = usize::MAX;

        for i in 0..cards.len() {
            if let Some(j) = indices.insert(cards[i], i) {
                ret = ret.min(i - j + 1);
            }
        }

        if ret == usize::MAX {
            return -1;
        }

        ret as i32
    }
}
```
