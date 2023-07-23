impl Solution {
    pub fn reformat(s: String) -> String {
        let (mut digits, mut letters): (Vec<u8>, Vec<u8>) =
            s.bytes().partition(|ch| ch.is_ascii_digit());
        let mut ret = vec![];

        let mut iterator = if digits.len() == letters.len() + 1 {
            ret.push(digits.pop().unwrap());
            letters.iter().zip(digits.iter())
        } else if digits.len() + 1 == letters.len() {
            ret.push(letters.pop().unwrap());
            digits.iter().zip(letters.iter())
        } else if digits.len() == letters.len() {
            digits.iter().zip(letters.iter())
        } else {
            [].iter().zip([].iter())
        };

        while let Some((&ch0, &ch1)) = iterator.next() {
            ret.push(ch0);
            ret.push(ch1);
        }

        String::from_utf8(ret).unwrap()
    }
}
