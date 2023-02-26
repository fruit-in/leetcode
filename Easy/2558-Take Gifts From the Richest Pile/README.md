# 2558. Take Gifts From the Richest Pile
You are given an integer array `gifts` denoting the number of gifts in various piles. Every second, you do the following:

* Choose the pile with the maximum number of gifts.
* If there is more than one pile with the maximum number of gifts, choose any.
* Leave behind the floor of the square root of the number of gifts in the pile. Take the rest of the gifts.

Return *the number of gifts remaining after* `k` *seconds*.

#### Example 1:
<pre>
<strong>Input:</strong> gifts = [25,64,9,4,100], k = 4
<strong>Output:</strong> 29
<strong>Explanation:</strong>
The gifts are taken in the following way:
- In the first second, the last pile is chosen and 10 gifts are left behind.
- Then the second pile is chosen and 8 gifts are left behind.
- After that the first pile is chosen and 5 gifts are left behind.
- Finally, the last pile is chosen again and 3 gifts are left behind.
The final remaining gifts are [5,8,9,4,3], so the total number of gifts remaining is 29.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> gifts = [1,1,1,1], k = 4
<strong>Output:</strong> 4
<strong>Explanation:</strong>
In this case, regardless which pile you choose, you have to leave behind 1 gift in each pile.
That is, you can't take any pile with you.
So, the total gifts remaining are 4.
</pre>

#### Constraints:
* <code>1 <= gifts.length <= 10<sup>3</sup></code>
* <code>1 <= gifts[i] <= 10<sup>9</sup></code>
* <code>1 <= k <= 10<sup>3</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut gifts = BinaryHeap::from(gifts);

        for _ in 0..k {
            let mut x = gifts.peek_mut().unwrap();
            *x = (*x as f64).sqrt() as i32;
        }

        gifts.iter().map(|&x| x as i64).sum()
    }
}
```
