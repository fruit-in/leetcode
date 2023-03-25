impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let mut i = 0;
        let mut j = 0;
        let mut words1 = sentence1.split_whitespace().collect::<Vec<_>>();
        let mut words2 = sentence2.split_whitespace().collect::<Vec<_>>();
        if words1.len() > words2.len() {
            let temp = words1;
            words1 = words2;
            words2 = temp;
        }

        while i < words1.len() && words1[i] == words2[i] {
            i += 1;
        }
        while j < words1.len() && words1[words1.len() - 1 - j] == words2[words2.len() - 1 - j] {
            j += 1;
        }

        i + j >= words1.len()
    }
}
