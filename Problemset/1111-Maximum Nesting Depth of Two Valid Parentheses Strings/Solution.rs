impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut count0 = 0;
        let mut count1 = 0;
        let mut answer = vec![0; seq.len()];

        for (i, ch) in seq.chars().enumerate() {
            match (ch, count0 < count1) {
                ('(', true) => count0 += 1,
                (')', false) => count0 -= 1,
                (_, true) => count1 -= 1,
                (_, false) => count1 += 1,
            }

            answer[i] = ((ch == '(') ^ (count0 < count1)) as i32;
        }

        answer
    }
}
