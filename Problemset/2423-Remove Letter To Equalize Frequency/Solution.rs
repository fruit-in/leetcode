impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let word = word.as_bytes();

        for i in 0..word.len() {
            let mut count = [0; 26];

            for j in (0..i).chain(i + 1..word.len()) {
                count[(word[j] - b'a') as usize] += 1;
            }

            let x = *count.iter().find(|&&x| x > 0).unwrap();

            if count.iter().all(|&c| c == 0 || c == x) {
                return true;
            }
        }

        false
    }
}
