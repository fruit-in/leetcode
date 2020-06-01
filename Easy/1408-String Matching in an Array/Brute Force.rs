impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut ret = Vec::new();

        for i in 0..words.len() {
            for j in 0..words.len() {
                if j != i && words[j].contains(&words[i]) {
                    ret.push(words[i].clone());
                    break;
                }
            }
        }

        ret
    }
}
