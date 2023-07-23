impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prefix_sum = vec![0; words.len() + 1];
        let mut ret = vec![0; queries.len()];

        for i in 0..words.len() {
            prefix_sum[i + 1] = prefix_sum[i];

            if words[i].starts_with(|c| "aeiou".contains(c))
                && words[i].ends_with(|c| "aeiou".contains(c))
            {
                prefix_sum[i + 1] += 1;
            }
        }

        for i in 0..queries.len() {
            ret[i] = prefix_sum[queries[i][1] as usize + 1] - prefix_sum[queries[i][0] as usize];
        }

        ret
    }
}
