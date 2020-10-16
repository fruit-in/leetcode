# 1561. Maximum Number of Coins You Can Get
There are 3n piles of coins of varying size, you and your friends will take piles of coins as follows:
* In each step, you will choose **any** 3 piles of coins (not necessarily consecutive).
* Of your choice, Alice will pick the pile with the maximum number of coins.
* You will pick the next pile with maximum number of coins.
* Your friend Bob will pick the last pile.
* Repeat until there are no more piles of coins.

Given an array of integers `piles` where `piles[i]` is the number of coins in the `i`<sup>`th`</sup> pile.

Return the maximum number of coins which you can have.

#### Example 1:
<pre>
<b>Input:</b> piles = [2,4,1,2,7,8]
<b>Output:</b> 9
<b>Explanation:</b> Choose the triplet (2, 7, 8), Alice Pick the pile with 8 coins, you the pile with <b>7</b> coins and Bob the last one.
Choose the triplet (1, 2, 4), Alice Pick the pile with 4 coins, you the pile with <b>2</b> coins and Bob the last one.
The maximum number of coins which you can have are: 7 + 2 = 9.
On the other hand if we choose this arrangement (1, <b>2</b>, 8), (2, <b>4</b>, 7) you only get 2 + 4 = 6 coins which is not optimal.
</pre>

#### Example 2:
<pre>
<b>Input:</b> piles = [2,4,5]
<b>Output:</b> 4
</pre>

#### Example 3:
<pre>
<b>Input:</b> piles = [9,8,7,6,5,1,2,3,4]
<b>Output:</b> 18
</pre>

#### Constraints:
* `3 <= piles.length <= 10^5`
* `piles.length % 3 == 0`
* `1 <= piles[i] <= 10^4`

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn max_coins(piles: Vec<i32>) -> i32 {
        let n = piles.len() / 3;
        let mut piles = piles;
        piles.sort_unstable();

        piles.iter().skip(n).step_by(2).sum()
    }
}
```
