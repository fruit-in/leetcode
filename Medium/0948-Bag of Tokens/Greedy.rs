impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, p: i32) -> i32 {
        if tokens.len() == 0 {
            return 0;
        }

        let mut tokens = tokens;
        let mut p = p;
        let mut i = 0;
        let mut j = tokens.len() - 1;
        let mut score = 0;
        let mut ret = 0;
        tokens.sort_unstable();

        while i <= j {
            if p >= tokens[i] {
                p -= tokens[i];
                score += 1;
                ret = ret.max(score);
                i += 1;
            } else if score > 0 {
                p += tokens[j];
                score -= 1;
                j -= 1;
            } else {
                break;
            }
        }

        ret
    }
}
