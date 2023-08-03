impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let mut s = s.bytes().rev().collect::<Vec<_>>();
        let mut t = vec![];
        let mut p = vec![];
        let mut count = [0; 26];

        for &ch in &s {
            count[(ch - b'a') as usize] += 1;
        }

        while let Some(ch0) = s.pop() {
            count[(ch0 - b'a') as usize] -= 1;
            t.push(ch0);

            while let Some(&ch1) = t.last() {
                if count.iter().take((ch1 - b'a') as usize).all(|&x| x == 0) {
                    p.push(t.pop().unwrap());
                } else {
                    break;
                }
            }
        }

        while let Some(ch) = t.pop() {
            p.push(ch);
        }

        String::from_utf8(p).unwrap()
    }
}
