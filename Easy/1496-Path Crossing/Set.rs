use std::collections::HashSet;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut visited = HashSet::new();
        let mut x = 0;
        let mut y = 0;
        visited.insert((x, y));

        for dir in path.bytes() {
            match dir {
                b'N' => y += 1,
                b'S' => y -= 1,
                b'E' => x += 1,
                _ => x -= 1,
            };
            if !visited.insert((x, y)) {
                return true;
            }
        }

        false
    }
}
