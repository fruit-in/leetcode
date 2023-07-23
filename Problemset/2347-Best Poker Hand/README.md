# 2347. Best Poker Hand
You are given an integer array `ranks` and a character array `suits`. You have `5` cards where the <code>i<sup>th</sup></code> card has a rank of `ranks[i]` and a suit of `suits[i]`.

The following are the types of **poker hands** you can make from best to worst:
1. `"Flush"`: Five cards of the same suit.
2. `"Three of a Kind"`: Three cards of the same rank.
3. `"Pair"`: Two cards of the same rank.
4. `"High Card"`: Any single card.

Return *a string representing the **best** type of **poker hand** you can make with the given cards*.

**Note** that the return values are **case-sensitive**.

#### Example 1:
<pre>
<strong>Input:</strong> ranks = [13,2,3,1,9], suits = ["a","a","a","a","a"]
<strong>Output:</strong> "Flush"
<strong>Explanation:</strong> The hand with all the cards consists of 5 cards with the same suit, so we have a "Flush".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> ranks = [4,4,2,4,4], suits = ["d","a","a","b","c"]
<strong>Output:</strong> "Three of a Kind"
<strong>Explanation:</strong> The hand with the first, second, and fourth card consists of 3 cards with the same rank, so we have a "Three of a Kind".
Note that we could also make a "Pair" hand but "Three of a Kind" is a better hand.
Also note that other cards could be used to make the "Three of a Kind" hand.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> ranks = [10,10,2,12,9], suits = ["a","b","c","a","d"]
<strong>Output:</strong> "Pair"
<strong>Explanation:</strong> The hand with the first and second card consists of 2 cards with the same rank, so we have a "Pair".
Note that we cannot make a "Flush" or a "Three of a Kind".
</pre>

#### Constraints:
* `ranks.length == suits.length == 5`
* `1 <= ranks[i] <= 13`
* `'a' <= suits[i] <= 'd'`
* No two cards have the same rank and suit.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def bestHand(self, ranks: List[int], suits: List[str]) -> str:
        ranks_counter = Counter(ranks)
        suits_counter = Counter(suits)

        if len(suits_counter) == 1:
            return "Flush"
        elif max(ranks_counter.values()) > 2:
            return "Three of a Kind"
        elif max(ranks_counter.values()) > 1:
            return "Pair"
        else:
            return "High Card"
```
