use std::collections::HashMap;

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let mut count = HashMap::new();
        let mut answer = vec![0; puzzles.len()];

        for word in words {
            let bits = word.bytes().fold(0_i32, |b, c| b | (1 << (c - b'a')));

            if bits.count_ones() < 8 {
                *count.entry(bits).or_insert(0) += 1;
            }
        }

        for i in 0..answer.len() {
            let puzzle = puzzles[i].as_bytes();
            let mut combins = vec![1 << (puzzle[0] - b'a')];
            answer[i] += count.get(&combins[0]).unwrap_or(&0);

            for j in 1..puzzle.len() {
                for k in 0..combins.len() {
                    let combin = combins[k] | (1 << (puzzle[j] - b'a'));
                    combins.push(combin);
                    answer[i] += count.get(&combin).unwrap_or(&0);
                }
            }
        }

        answer
    }
}
