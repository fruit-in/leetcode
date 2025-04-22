impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        let mut lis = vec![];

        envelopes.sort_unstable_by_key(|e| (e[0], -e[1]));

        for i in 0..envelopes.len() {
            if let Err(j) = lis.binary_search(&envelopes[i][1]) {
                if j == lis.len() {
                    lis.push(envelopes[i][1]);
                } else {
                    lis[j] = envelopes[i][1];
                }
            }
        }

        lis.len() as i32
    }
}
