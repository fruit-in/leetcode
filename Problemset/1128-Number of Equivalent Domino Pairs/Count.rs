impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut cnt = [0; 100];
        let mut ret = 0;
        for domino in dominoes {
            let tens = domino[0].min(domino[1]) as usize;
            let units = domino[0].max(domino[1]) as usize;
            ret += cnt[tens * 10 + units];
            cnt[tens * 10 + units] += 1;
        }
        ret
    }
}
