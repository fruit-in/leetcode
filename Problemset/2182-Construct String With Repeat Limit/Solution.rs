impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut count = vec![0; 26];
        let mut repeat = 0;
        let mut flag = true;
        let mut ret = vec![];

        for c in s.bytes() {
            count[(c - b'a') as usize] += 1;
        }

        while flag {
            flag = false;

            for c in (b'a'..=b'z').rev() {
                if count[(c - b'a') as usize] > 0 {
                    if *ret.last().unwrap_or(&b' ') != c || repeat < repeat_limit {
                        if *ret.last().unwrap_or(&b' ') != c {
                            repeat = 0;
                        }
                        count[(c - b'a') as usize] -= 1;
                        repeat += 1;
                        flag = true;
                        ret.push(c);
                        break;
                    }
                }
            }
        }

        String::from_utf8(ret).unwrap()
    }
}
