# 2038. Remove Colored Pieces if Both Neighbors are the Same Color
There are `n` pieces arranged in a line, and each piece is colored either by `'A'` or by `'B'`. You are given a string `colors` of length `n` where `colors[i]` is the color of the <code>i<sup>th</sup></code> piece.

Alice and Bob are playing a game where they take **alternating turns** removing pieces from the line. In this game, Alice moves **first**.

* Alice is only allowed to remove a piece colored `'A'` if **both its neighbors** are also colored `'A'`. She is **not allowed** to remove pieces that are colored `'B'`.
* Bob is only allowed to remove a piece colored `'B'` if **both its neighbors** are also colored `'B'`. He is **not allowed** to remove pieces that are colored `'A'`.
* Alice and Bob **cannot** remove pieces from the edge of the line.
* If a player cannot make a move on their turn, that player **loses** and the other player **wins**.

Assuming Alice and Bob play optimally, return `true` *if Alice wins, or return* `false` *if Bob wins*.

#### Example 1:
<pre>
<strong>Input:</strong> colors = "AAABABB"
<strong>Output:</strong> true
<strong>Explanation:</strong>
AAABABB -> AABABB
Alice moves first.
She removes the second 'A' from the left since that is the only 'A' whose neighbors are both 'A'.

Now it's Bob's turn.
Bob cannot make a move on his turn since there are no 'B's whose neighbors are both 'B'.
Thus, Alice wins, so return true.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> colors = "AA"
<strong>Output:</strong> false
<strong>Explanation:</strong>
Alice has her turn first.
There are only two 'A's and both are on the edge of the line, so she cannot move on her turn.
Thus, Bob wins, so return false.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> colors = "ABBBBBBBAAA"
<strong>Output:</strong> false
<strong>Explanation:</strong>
ABBBBBBBAAA -> ABBBBBBBAA
Alice moves first.
Her only option is to remove the second to last 'A' from the right.

ABBBBBBBAA -> ABBBBBBAA
Next is Bob's turn.
He has many options for which 'B' piece to remove. He can pick any.

On Alice's second turn, she has no more pieces that she can remove.
Thus, Bob wins, so return false.
</pre>

#### Constraints:
* <code>1 <= colors.length <= 10<sup>5</sup></code>
* `colors` consists of only the letters `'A'` and `'B'`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        let colors = colors.as_bytes();
        let mut count = 1;
        let mut count_a = 0;
        let mut count_b = 0;

        for i in 1..colors.len() + 1 {
            if i >= colors.len() || colors[i] != colors[i - 1] {
                if colors[i - 1] == b'A' {
                    count_a += (count - 2).max(0);
                } else {
                    count_b += (count - 2).max(0);
                }

                count = 0;
            }

            count += 1;
        }

        count_a > count_b
    }
}
```
