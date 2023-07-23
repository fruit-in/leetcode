impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let mut start_pieces = start
            .chars()
            .enumerate()
            .filter(|(_, c)| *c != '_')
            .collect::<Vec<_>>();
        let mut target_pieces = target
            .chars()
            .enumerate()
            .filter(|(_, c)| *c != '_')
            .collect::<Vec<_>>();

        if start_pieces.len() != target_pieces.len() {
            return false;
        }

        for i in 0..start_pieces.len() {
            match (start_pieces[i], target_pieces[i]) {
                ((j, 'L'), (k, 'L')) if j >= k => (),
                ((j, 'R'), (k, 'R')) if j <= k => (),
                _ => return false,
            }
        }

        true
    }
}
