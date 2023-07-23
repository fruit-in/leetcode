impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let mut count0 = [0; 26];
        let mut ret = 0;

        for &c in &letters {
            count0[c as usize - 97] += 1;
        }

        for x in 0..2_i32.pow(words.len() as u32) {
            let mut count1 = count0;
            let mut s = 0;
            let mut flag = false;

            for i in 0..words.len() {
                if flag {
                    break;
                }

                if (1 << i) & x != 0 {
                    for c in words[i].bytes() {
                        if count1[(c - b'a') as usize] <= 0 {
                            flag = true;
                            s = i32::MIN;
                            break;
                        }

                        count1[(c - b'a') as usize] -= 1;
                        s += score[(c - b'a') as usize];
                    }
                }
            }

            ret = ret.max(s);
        }

        ret
    }
}
