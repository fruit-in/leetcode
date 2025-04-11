impl Solution {
    pub fn count_palindromes(s: String) -> i32 {
        let s = s.bytes().map(|c| (c - b'0') as usize).collect::<Vec<_>>();
        let mut count1 = [0_i64; 10];
        let mut count2l = vec![[0; 100]; s.len()];
        let mut count2r = vec![[0; 100]; s.len()];
        let mut ret = 0;

        count1[s[0]] = 1;
        for i in 1..s.len() {
            count2l[i] = count2l[i - 1];
            for j in 0..10 {
                count2l[i][s[i] * 10 + j] = (count2l[i][s[i] * 10 + j] + count1[j]) % 1_000_000_007;
            }
            count1[s[i]] += 1;
        }

        count1 = [0; 10];
        count1[s[s.len() - 1]] = 1;
        for i in (0..s.len() - 1).rev() {
            count2r[i] = count2r[i + 1];
            for j in 0..10 {
                count2r[i][s[i] * 10 + j] = (count2r[i][s[i] * 10 + j] + count1[j]) % 1_000_000_007;
            }
            count1[s[i]] += 1;
        }

        for i in 2..s.len().saturating_sub(2) {
            for j in 0..100 {
                ret = (ret + count2l[i - 1][j] * count2r[i + 1][j] % 1_000_000_007) % 1_000_000_007;
            }
        }

        ret as i32
    }
}
