# 1423. Maximum Points You Can Obtain from Cards
There are several cards **arranged in a row**, and each card has an associated number of points The points are given in the integer array `cardPoints`.

In one step, you can take one card from the beginning or from the end of the row. You have to take exactly `k` cards.

Your score is the sum of the points of the cards you have taken.

Given the integer array `cardPoints` and the integer `k`, return the *maximum score* you can obtain.

#### Example 1:
<pre>
<strong>Input:</strong> cardPoints = [1,2,3,4,5,6,1], k = 3
<strong>Output:</strong> 12
<strong>Explanation:</strong> After the first step, your score will always be 1. However, choosing the rightmost card first will maximize your total score. The optimal strategy is to take the three cards on the right, giving a final score of 1 + 6 + 5 = 12.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> cardPoints = [2,2,2], k = 2
<strong>Output:</strong> 4
<strong>Explanation:</strong> Regardless of which two cards you take, your score will always be 4.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> cardPoints = [9,7,7,9,7,7,9], k = 7
<strong>Output:</strong> 55
<strong>Explanation:</strong> You have to take all the cards. Your score is the sum of points of all cards.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> cardPoints = [1,1000,1], k = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> You cannot take the card in the middle. Your best score is 1.
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> cardPoints = [1,79,80,1,1,1,200,1], k = 3
<strong>Output:</strong> 202
</pre>

#### Constraints:
* `1 <= cardPoints.length <= 10^5`
* `1 <= cardPoints[i] <= 10^4`
* `1 <= k <= cardPoints.length`

## Solutions (Rust)

### 1. Sliding Window
```Rust
impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut score = card_points.iter().take(k).sum::<i32>();
        let mut ret = score;

        for i in 1..=k {
            score += card_points[card_points.len() - i] - card_points[k - i];
            ret = ret.max(score);
        }

        ret
    }
}
```
