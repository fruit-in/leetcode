impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        let value1 = first_word
            .bytes()
            .map(|c| c - b'a')
            .fold(0, |acc, x| acc * 10 + x as i32);
        let value2 = second_word
            .bytes()
            .map(|c| c - b'a')
            .fold(0, |acc, x| acc * 10 + x as i32);
        let target = target_word
            .bytes()
            .map(|c| c - b'a')
            .fold(0, |acc, x| acc * 10 + x as i32);

        value1 + value2 == target
    }
}
