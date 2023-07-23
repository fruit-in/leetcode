use std::collections::HashMap;

struct TopVotedCandidate {
    winners: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let mut counter = HashMap::new();
        let mut max = 0;
        let mut winner = 0;
        let mut winners = vec![];

        for (p, t) in persons.into_iter().zip(times.into_iter()) {
            let c = counter.entry(p).or_insert(0);
            *c += 1;
            if *c >= max {
                max = *c;
                winner = p;
            }
            winners.push((t, winner));
        }

        Self { winners }
    }

    fn q(&self, t: i32) -> i32 {
        match self.winners.binary_search_by_key(&t, |&(time, _)| time) {
            Ok(i) => self.winners[i].1,
            Err(i) => self.winners[i - 1].1,
        }
    }
}

/**
 * Your TopVotedCandidate object will be instantiated and called as such:
 * let obj = TopVotedCandidate::new(persons, times);
 * let ret_1: i32 = obj.q(t);
 */
