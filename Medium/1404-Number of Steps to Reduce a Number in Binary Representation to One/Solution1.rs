use std::collections::VecDeque;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut bits = s.chars().map(|c| c == '1').collect::<VecDeque<_>>();
        let mut ret = 0;

        while bits.len() > 1 {
            if *bits.back().unwrap() {
                let mut carry = true;

                for i in (0..bits.len()).rev() {
                    if carry {
                        bits[i] = !bits[i];
                        carry = !bits[i];
                    } else {
                        break;
                    }
                }

                if carry {
                    bits.push_front(true);
                }
            } else {
                bits.pop_back();
            }

            ret += 1
        }

        ret
    }
}
