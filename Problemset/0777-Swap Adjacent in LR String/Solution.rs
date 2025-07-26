impl Solution {
    pub fn can_transform(start: String, result: String) -> bool {
        let start = start.as_bytes();
        let result = result.as_bytes();
        let mut i = 0;
        let mut j = 0;

        loop {
            while i < start.len() && start[i] == b'X' {
                i += 1;
            }
            while j < result.len() && result[j] == b'X' {
                j += 1;
            }

            if i == start.len() || j == result.len() {
                return i == start.len() && j == result.len();
            } else if (start[i] == b'L' && result[j] == b'L' && i >= j)
                || (start[i] == b'R' && result[j] == b'R' && i <= j)
            {
                i += 1;
                j += 1;
            } else {
                return false;
            }
        }

        unreachable!()
    }
}
