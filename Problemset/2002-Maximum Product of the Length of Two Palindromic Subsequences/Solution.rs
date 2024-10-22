impl Solution {
    pub fn max_product(s: String) -> i32 {
        let s = s.as_bytes();
        let mut palindromic_subs = vec![];
        let mut ret = 0;

        for x in 1_i32..(1 << s.len()) {
            let mut sub = vec![];

            for i in 0..s.len() {
                if x & (1 << i) != 0 {
                    sub.push(s[i]);
                }
            }

            for i in 0..=sub.len() / 2 {
                if sub[i] != sub[sub.len() - 1 - i] {
                    break;
                }
                if i == sub.len() / 2 {
                    for y in &palindromic_subs {
                        if x & y == 0 {
                            ret = ret.max(x.count_ones() * y.count_ones());
                        }
                    }
                    palindromic_subs.push(x);
                }
            }
        }

        ret as i32
    }
}
