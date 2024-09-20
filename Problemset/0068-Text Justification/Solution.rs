impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut last_len = max_width;
        let mut lines = vec![];
        let mut ret = vec![];

        for word in words {
            if last_len + 1 + word.len() > max_width {
                last_len = word.len();
                lines.push(vec![word]);
            } else {
                last_len += 1 + word.len();
                lines.last_mut().unwrap().push(word);
            }
        }

        for i in 0..lines.len() {
            let words_len = lines[i].iter().map(|w| w.len()).sum::<usize>();
            let mut spaces_len = (max_width - words_len) / (lines[i].len().max(2) - 1);
            let mut left_len = (max_width - words_len) % (lines[i].len().max(2) - 1);
            let mut line = vec![];

            if i == lines.len() - 1 {
                spaces_len = 1;
                left_len = 0;
            }

            for j in 0..lines[i].len() {
                line.push(lines[i][j].clone());
                if j < lines[i].len() - 1 {
                    if left_len > 0 {
                        line.push(" ".repeat(spaces_len + 1));
                        left_len -= 1;
                    } else {
                        line.push(" ".repeat(spaces_len));
                    }
                }
            }

            let mut line = line.concat();

            if line.len() < max_width {
                line += &" ".repeat(max_width - line.len());
            }

            ret.push(line);
        }

        ret
    }
}
