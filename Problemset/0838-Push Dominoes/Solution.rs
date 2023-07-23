impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut dominoes = format!("L{}R", dominoes).into_bytes();
        let mut i = 0;

        for j in 1..dominoes.len() {
            if dominoes[j] != b'.' {
                match (dominoes[i], dominoes[j]) {
                    (b'L', b'L') => dominoes[i..j].iter_mut().for_each(|x| *x = b'L'),
                    (b'R', b'R') => dominoes[i..j].iter_mut().for_each(|x| *x = b'R'),
                    (b'R', b'L') => {
                        for k in 1..(j - i + 1) / 2 {
                            dominoes[i + k] = b'R';
                            dominoes[j - k] = b'L';
                        }
                    }
                    _ => (),
                }

                i = j;
            }
        }

        dominoes.remove(0);
        dominoes.pop();

        String::from_utf8(dominoes).unwrap()
    }
}
