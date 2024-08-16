impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut masks = Vec::new();
        let mut ret = 0;

        for s in &arr {
            let mask = s.bytes().fold(0_i32, |acc, c| acc | (1 << (c - b'a')));

            if mask.count_ones() == s.len() as u32 {
                masks.push(mask);
            }
        }

        for x in 0..2_i32.pow(masks.len() as u32) {
            let mut mask = 0;
            let mut flag = true;

            for i in 0..masks.len() {
                if x & (1 << i) != 0 {
                    if mask ^ masks[i] != mask | masks[i] {
                        flag = false;
                        break;
                    }

                    mask |= masks[i];
                }
            }

            if flag {
                ret = ret.max(mask.count_ones());
            }
        }

        ret as i32
    }
}
