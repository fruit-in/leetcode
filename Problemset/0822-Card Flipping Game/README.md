# 822. Card Flipping Game
On a table are `N` cards, with a positive integer printed on the front and back of each card (possibly different).

We flip any number of cards, and after we choose one card.

If the number `X` on the back of the chosen card is not on the front of any card, then this number X is good.

What is the smallest number that is good?  If no number is good, output `0`.

Here, `fronts[i]` and `backs[i]` represent the number on the front and back of card `i`.

A flip swaps the front and back numbers, so the value on the front is now on the back and vice versa.

#### Example:
<pre>
<strong>Input:</strong> fronts = [1,2,4,4,7], backs = [1,3,4,1,3]
<strong>Output:</strong> 2
<strong>Explanation:</strong> If we flip the second card, the fronts are [1,3,4,4,7] and the backs are [1,2,4,1,3].
We choose the second card, which has number 2 on the back, and it isn't on the front of any card, so 2 is good.
</pre>

#### Constraints:
1. `1 <= fronts.length == backs.length <= 1000`.
2. `1 <= fronts[i] <= 2000`.
3. `1 <= backs[i] <= 2000`.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
        let mut good = HashSet::new();
        let mut ban = HashSet::new();

        for i in 0..backs.len() {
            if fronts[i] == backs[i] {
                good.remove(&backs[i]);
                ban.insert(backs[i]);
            } else {
                if !ban.contains(&fronts[i]) {
                    good.insert(fronts[i]);
                }
                if !ban.contains(&backs[i]) {
                    good.insert(backs[i]);
                }
            }
        }

        good.into_iter().min().unwrap_or(0)
    }
}
```
