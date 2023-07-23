impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut groups = Vec::new();
        let mut start = 0;
        let mut count = 0;
        let mut cur_ch = ' ';
        for ch in s.chars() {
            if ch == cur_ch {
                count += 1;
            } else if ch != cur_ch {
                if count >= 3 {
                    groups.push(vec![start, start + count - 1]);
                }
                start += count;
                count = 1;
                cur_ch = ch;
            }
        }
        if count >= 3 {
            groups.push(vec![start, start + count - 1]);
        }
        groups
    }
}
