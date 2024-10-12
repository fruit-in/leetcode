impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        let mut ret = vec![];

        for wordq in &queries {
            for wordd in &dictionary {
                if wordq
                    .chars()
                    .zip(wordd.chars())
                    .filter(|(a, b)| a != b)
                    .count()
                    <= 2
                {
                    ret.push(wordq.clone());
                    break;
                }
            }
        }

        ret
    }
}
