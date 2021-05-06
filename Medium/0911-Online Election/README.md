# 911. Online Election
In an election, the `i`-th vote was cast for `persons[i]` at time `times[i]`.

Now, we would like to implement the following query function: `TopVotedCandidate.q(int t)` will return the number of the person that was leading the election at time `t`.

Votes cast at time `t` will count towards our query.  In the case of a tie, the most recent vote (among tied candidates) wins.

#### Example 1:
<pre>
<strong>Input:</strong> ["TopVotedCandidate","q","q","q","q","q","q"], [[[0,1,1,0,0,1,0],[0,5,10,15,20,25,30]],[3],[12],[25],[15],[24],[8]]
<strong>Output:</strong> [null,0,1,1,0,0,1]
<strong>Explanation:</strong>
At time 3, the votes are [0], and 0 is leading.
At time 12, the votes are [0,1,1], and 1 is leading.
At time 25, the votes are [0,1,1,0,0,1], and 1 is leading (as ties go to the most recent vote.)
This continues for 3 more queries at time 15, 24, and 8.
</pre>

#### Note:
1. `1 <= persons.length = times.length <= 5000`
2. `0 <= persons[i] <= persons.length`
3. `times` is a strictly increasing array with all elements in `[0, 10^9]`.
4. `TopVotedCandidate.q` is called at most `10000` times per test case.
5. `TopVotedCandidate.q(int t)` is always called with `t >= times[0]`.

## Solutions (Rust)

### 1. Binary Search
```Rust
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
```
