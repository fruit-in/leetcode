impl Solution {
    pub fn max_rep_opt1(text: String) -> i32 {
        let text = text.as_bytes();
        let mut ranges = vec![vec![]; 26];
        let mut ret = 1;

        for i in 0..text.len() {
            if i == 0 || text[i] != text[i - 1] {
                ranges[(text[i] - b'a') as usize].push((i, i));
            } else {
                ranges[(text[i] - b'a') as usize].last_mut().unwrap().1 = i;
            }
        }

        for i in 0..ranges.len() {
            for j in 0..ranges[i].len() {
                ret = ret.max(ranges[i][j].1 - ranges[i][j].0 + 1 + (ranges[i].len() > 1) as usize);

                if j > 0 && ranges[i][j].0 == ranges[i][j - 1].1 + 2 {
                    ret = ret
                        .max(ranges[i][j].1 - ranges[i][j - 1].0 + (ranges[i].len() > 2) as usize);
                }
            }
        }

        ret as i32
    }
}
