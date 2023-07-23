impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut mat = [[0; 26]; 26];
        let mut flag = 0;
        let mut ret = 0;

        for word in words {
            mat[(word.as_bytes()[0] - b'a') as usize][(word.as_bytes()[1] - b'a') as usize] += 1;
        }

        for i in 0..26 {
            flag |= mat[i][i] % 2;
            ret += mat[i][i] / 2 * 4;
            for j in (i + 1)..26 {
                ret += 4 * mat[i][j].min(mat[j][i])
            }
        }

        ret + 2 * flag
    }
}
