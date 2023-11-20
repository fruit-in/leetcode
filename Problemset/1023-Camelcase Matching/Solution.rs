impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let pattern = pattern.as_bytes();
        let mut ret = vec![true; queries.len()];

        for i in 0..queries.len() {
            let query = queries[i].as_bytes();
            let mut j = 0;

            for k in 0..query.len() {
                if j < pattern.len() && query[k] == pattern[j] {
                    j += 1;
                } else if query[k].is_ascii_uppercase() {
                    ret[i] = false;
                    break;
                }
            }

            ret[i] &= j == pattern.len();
        }

        ret
    }
}
