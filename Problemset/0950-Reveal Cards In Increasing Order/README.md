# 950. Reveal Cards In Increasing Order
In a deck of cards, every card has a unique integer.  You can order the deck in any order you want.

Initially, all the cards start face down (unrevealed) in one deck.

Now, you do the following steps repeatedly, until all cards are revealed:
1. Take the top card of the deck, reveal it, and take it out of the deck.
2. If there are still cards in the deck, put the next top card of the deck at the bottom of the deck.
3. If there are still unrevealed cards, go back to step 1.  Otherwise, stop.

Return an ordering of the deck that would reveal the cards in **increasing order.**

The first entry in the answer is considered to be the top of the deck.

#### Example 1:
<pre>
<strong>Input:</strong> [17,13,11,2,3,5,7]
<strong>Output:</strong> [2,13,3,11,5,17,7]
<strong>Explanation:</strong>
We get the deck in the order [17,13,11,2,3,5,7] (this order doesn't matter), and reorder it.
After reordering, the deck starts as [2,13,3,11,5,17,7], where 2 is the top of the deck.
We reveal 2, and move 13 to the bottom.  The deck is now [3,11,5,17,7,13].
We reveal 3, and move 11 to the bottom.  The deck is now [5,17,7,13,11].
We reveal 5, and move 17 to the bottom.  The deck is now [7,13,11,17].
We reveal 7, and move 13 to the bottom.  The deck is now [11,17,13].
We reveal 11, and move 17 to the bottom.  The deck is now [13,17].
We reveal 13, and move 17 to the bottom.  The deck is now [17].
We reveal 17.
Since all the cards revealed are in increasing order, the answer is correct.
</pre>

#### Note:
1. ```1 <= A.length <= 1000```
2. ```1 <= A[i] <= 10^6```
3. ```A[i] != A[j]``` for all ```i != j```

## Solutions (Rust)

### 1. Simulation
```Rust
impl Solution {
    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        let mut deck = deck;
        deck.sort_unstable();
        let mut ret = Vec::new();

        for _ in 0..deck.len() {
            if let Some(last) = ret.pop() {
                ret.insert(0, last);
            }
            ret.insert(0, deck.pop().unwrap());
        }

        ret
    }
}
```
