impl Solution {
    pub fn count_anagrams(s: String) -> i32 {
        fn power(x: i64, exp: u32) -> i64 {
            if exp == 0 {
                1
            } else if exp % 2 == 0 {
                power(x, exp / 2).pow(2) % 1_000_000_007
            } else {
                x * power(x, exp - 1) % 1_000_000_007
            }
        }

        let mut factorials = vec![1];
        let mut ret = 1_i64;

        for word in s.split_whitespace() {
            let mut count = [0; 26];
            let mut length = 0;

            for c in word.bytes() {
                count[(c - b'a') as usize] += 1;
            }

            for c in count {
                length += c;
                while length >= factorials.len() {
                    let x = (*factorials.last().unwrap() * factorials.len() as i64) % 1_000_000_007;
                    factorials.push(x);
                }
                ret = (ret * factorials[length]) % 1_000_000_007;
                ret = (ret * power(factorials[c], 1_000_000_005)) % 1_000_000_007;
                ret = (ret * power(factorials[length - c], 1_000_000_005)) % 1_000_000_007;
            }
        }

        ret as i32
    }
}
