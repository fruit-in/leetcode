impl Solution {
    pub fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32> {
        let s = s.as_bytes();
        let mut answer = vec![0; s.len()];

        for i in 0..s.len() {
            let mut pos = (start_pos[0], start_pos[1]);

            for j in i..s.len() {
                if s[j] == b'L' && pos.1 > 0 {
                    pos.1 -= 1;
                } else if s[j] == b'R' && pos.1 < n - 1 {
                    pos.1 += 1;
                } else if s[j] == b'U' && pos.0 > 0 {
                    pos.0 -= 1;
                } else if s[j] == b'D' && pos.0 < n - 1 {
                    pos.0 += 1;
                } else {
                    break;
                }

                answer[i] += 1;
            }
        }

        answer
    }
}
