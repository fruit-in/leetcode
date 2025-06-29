impl Solution {
    pub fn is_it_possible(word1: String, word2: String) -> bool {
        let mut count1 = [0; 26];
        let mut count2 = [0; 26];
        let mut distinct1 = 0;
        let mut distinct2 = 0;

        for c in word1.bytes() {
            count1[(c - b'a') as usize] += 1;
            if count1[(c - b'a') as usize] == 1 {
                distinct1 += 1;
            }
        }
        for c in word2.bytes() {
            count2[(c - b'a') as usize] += 1;
            if count2[(c - b'a') as usize] == 1 {
                distinct2 += 1;
            }
        }

        for i in 0..26 {
            if count1[i] == 0 {
                continue;
            }

            for j in 0..26 {
                if count2[j] == 0 {
                    continue;
                }
                if i == j {
                    if distinct1 == distinct2 {
                        return true;
                    } else {
                        continue;
                    }
                }

                let mut tmp1 = distinct1;
                let mut tmp2 = distinct2;

                if count1[i] == 1 {
                    tmp1 -= 1;
                }
                if count2[i] == 0 {
                    tmp2 += 1;
                }
                if count1[j] == 0 {
                    tmp1 += 1;
                }
                if count2[j] == 1 {
                    tmp2 -= 1;
                }

                if tmp1 == tmp2 {
                    return true;
                }
            }
        }

        false
    }
}
