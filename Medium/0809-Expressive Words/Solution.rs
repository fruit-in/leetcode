impl Solution {
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        let s = Self::group(&s);

        words
            .iter()
            .map(|w| Self::group(w))
            .filter(|w| {
                s.len() == w.len()
                    && s.iter().zip(w.iter()).all(|(&(c0, x), &(c1, y))| {
                        c0 == c1 && ((y == 1 && x != 2) || (y != 1 && x >= y))
                    })
            })
            .count() as i32
    }

    fn group(s: &str) -> Vec<(char, i32)> {
        let mut ret = vec![];

        for c0 in s.chars() {
            match ret.last_mut() {
                Some((c1, x)) if c0 == *c1 => *x += 1,
                _ => ret.push((c0, 1)),
            }
        }

        ret
    }
}
