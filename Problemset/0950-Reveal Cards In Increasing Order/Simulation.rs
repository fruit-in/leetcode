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
